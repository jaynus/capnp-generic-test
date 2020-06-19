use capnp::capability::Promise;
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};

use crate::generic_client_capnp::{a, b, worker};

use futures::{AsyncReadExt, FutureExt};
use std::net::ToSocketAddrs;

#[derive(Clone, Default)]
struct AImpl;
impl a::Server for AImpl {
    fn test(
        &mut self,
        params: a::TestParams,
        mut results: a::TestResults,
    ) -> Promise<(), ::capnp::Error> {
        results.get().set_reply(&format!(
            "A, {}!",
            params.get().unwrap().get_request().unwrap()
        ));

        Promise::ok(())
    }
}

#[derive(Clone, Default)]
struct BImpl;
impl b::Server for BImpl {
    fn test(
        &mut self,
        params: b::TestParams,
        mut results: b::TestResults,
    ) -> Promise<(), ::capnp::Error> {
        results.get().set_reply(&format!(
            "A, {}!",
            params.get().unwrap().get_request().unwrap()
        ));

        Promise::ok(())
    }
}

struct WorkerImpl<T> {
    a: AImpl,
    b: BImpl,
    _marker: std::marker::PhantomData<T>,
}
impl<T> Default for WorkerImpl<T> {
    fn default() -> Self {
        Self {
            a: AImpl::default(),
            b: BImpl::default(),
            _marker: Default::default(),
        }
    }
}
impl<T> worker::Server<T> for WorkerImpl<T>
where
    T: for<'a> capnp::traits::Owned<'a>,
{
    fn get_interface(
        &mut self,
        params: worker::GetInterfaceParams<T>,
        mut results: worker::GetInterfaceResults<T>,
    ) -> Promise<(), ::capnp::Error> {
        results
            .get()
            .set_interface(capnp_rpc::new_client(self.a.clone()));

        Promise::ok(())
    }
}

#[derive(Default)]
struct WorkerImplWorking {
    a: AImpl,
    b: BImpl,
}
impl worker::Server<capnp::any_pointer::Owned> for WorkerImplWorking {
    fn get_interface(
        &mut self,
        params: worker::GetInterfaceParams<capnp::any_pointer::Owned>,
        mut results: worker::GetInterfaceResults<capnp::any_pointer::Owned>,
    ) -> Promise<(), ::capnp::Error> {
        results.get().init_interface().set_as_capability(
            capnp_rpc::new_client::<a::Client, AImpl>(self.a.clone())
                .client
                .hook,
        );

        Promise::ok(())
    }
}

pub fn main() {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() != 3 {
        println!("usage: {} server ADDRESS[:PORT]", args[0]);
        return;
    }

    let addr = args[2]
        .to_socket_addrs()
        .unwrap()
        .next()
        .expect("could not parse address");

    smol::block_on(async move {
        let listener = smol::Async::<std::net::TcpListener>::bind(&addr).unwrap();

        let worker_client: worker::Client<capnp::any_pointer::Owned> =
            capnp_rpc::new_client(WorkerImpl::default());

        loop {
            let (stream, _) = listener.accept().await.unwrap();

            let (reader, writer) = stream.split();
            let network = twoparty::VatNetwork::new(
                reader,
                writer,
                rpc_twoparty_capnp::Side::Server,
                Default::default(),
            );

            let rpc_system = RpcSystem::new(Box::new(network), Some(worker_client.clone().client));

            smol::Task::local(rpc_system.map(|_| ())).detach();
        }
    });
}

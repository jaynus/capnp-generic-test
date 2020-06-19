use crate::generic_client_capnp::worker;
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use std::net::ToSocketAddrs;

use futures::AsyncReadExt;

use futures::FutureExt;

pub fn main() {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() != 4 {
        println!("usage: {} client HOST:PORT MESSAGE", args[0]);
        return;
    }

    let addr = args[2]
        .to_socket_addrs()
        .unwrap()
        .next()
        .expect("could not parse address");

    let msg = args[3].to_string();

    smol::block_on(async move {
        let stream = smol::Async::<std::net::TcpStream>::connect(&addr)
            .await
            .unwrap();

        let (reader, writer) = stream.split();
        let rpc_network = Box::new(twoparty::VatNetwork::new(
            reader,
            writer,
            rpc_twoparty_capnp::Side::Client,
            Default::default(),
        ));
        let mut rpc_system = RpcSystem::new(rpc_network, None);
        let client: worker::Client<capnp::any_pointer::Owned> =
            rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);

        smol::Task::local(rpc_system.map(|_| ())).detach();

        let request = client.get_interface_request();
    });
}

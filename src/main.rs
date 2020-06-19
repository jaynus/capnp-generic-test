pub mod generic_client_capnp {
    include!(concat!(env!("OUT_DIR"), "/generic_client_capnp.rs"));
}

pub mod client;
pub mod server;

fn main() {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() >= 2 {
        match &args[1][..] {
            "client" => return client::main(),
            "server" => return server::main(),
            _ => (),
        }
    }

    println!("usage: {} [client | server] ADDRESS", args[0]);
}

fn main() {
    ::capnpc::CompilerCommand::new()
        .file("generic_client.capnp")
        .run()
        .unwrap();
}

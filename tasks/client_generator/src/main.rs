use anki_proto_gen::get_services;
use std::{env, fs};

mod client;

fn main() {
    let descriptors_bytes = include_bytes!(env!("DESCRIPTORS_BIN"));
    let (_, services) = get_services(
        prost_reflect::DescriptorPool::decode(&descriptors_bytes[..])
            .as_ref()
            .unwrap(),
    );

    let cwd = env::current_dir().unwrap();
    let target_dir = cwd.join("packages/rpc-client/src/generated/");
    client::generate(&target_dir, &services);
}

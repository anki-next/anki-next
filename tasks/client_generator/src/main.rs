use anki_proto_gen::{descriptors_path, get_services};
use std::{env, fs};

fn main() {
    let descriptors_bytes = include_bytes!(env!("DESCRIPTORS_BIN"));
    let (_, services) = get_services(
        prost_reflect::DescriptorPool::decode(&descriptors_bytes[..])
            .as_ref()
            .unwrap(),
    );

    let cwd = env::current_dir().unwrap();
    let target_dir = cwd.join("packages/rpc-client/src/generated");
    fs::create_dir_all(&target_dir).unwrap();
}

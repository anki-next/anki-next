use anki_proto_gen::{descriptors_path, get_services};

fn main() {
    let descriptors_bytes = include_bytes!(env!("DESCRIPTORS_BIN"));
    let (_, services) = get_services(
        prost_reflect::DescriptorPool::decode(&descriptors_bytes[..])
            .as_ref()
            .unwrap(),
    );
    let svcs = services
        .iter()
        .map(|svc| svc.name.clone())
        .collect::<Vec<_>>();
    println!("{:?}", services);
}

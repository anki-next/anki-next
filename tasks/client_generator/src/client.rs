use std::{fs, io::Write, path::PathBuf};

use anki_proto_gen::BackendService;

pub(crate) fn generate(dir: &PathBuf, services: &[BackendService]) {
    let file = dir.join("client.ts");

    let mut contents = vec![];

    contents.push(format!(
        "type ServiceName = {}",
        services
            .iter()
            .map(|svc| format!("\"{}\"", svc.name))
            .collect::<Vec<_>>()
            .join("|")
    ));

    let mut file = fs::File::create(file).unwrap();
    file.write_all(contents.join("\n").as_bytes()).unwrap();
}

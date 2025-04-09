use anki::backend::{Backend, init_backend};
use anki_proto::backend::BackendInit;
use prost::Message;

fn main() {
    let init = anki_proto::backend::BackendInit {
        preferred_langs: vec!["en".to_owned()],
        locale_folder_path: "test".to_owned(),
        server: false,
    };
    let backend = init_backend(&init.encode_to_vec()).unwrap();
}

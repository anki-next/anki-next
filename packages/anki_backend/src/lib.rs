use std::panic::catch_unwind;

use anki::{
    backend::{init_backend, Backend},
    error::Result,
};
use anki_proto::backend::{backend_error, BackendError, BackendInit};
use prost::Message;

pub struct AnkiBackend {
    backend: Backend,
}

impl AnkiBackend {
    pub fn new(preferred_langs: Vec<String>) -> Result<AnkiBackend, BackendError> {
        let init = BackendInit {
            preferred_langs,
            locale_folder_path: String::new(),
            server: false,
        };
        Ok(AnkiBackend {
            backend: init_backend(&init.encode_to_vec()).map_err(|err| BackendError {
                message: err,
                kind: backend_error::Kind::InvalidInput as i32,
                ..Default::default()
            })?,
        })
    }

    pub fn run_method(&self, service: u32, method: u32, input: &[u8]) -> Result<Vec<u8>, Vec<u8>> {
        match catch_unwind(|| self.backend.run_service_method(service, method, input)) {
            Ok(result) => result,
            Err(panic) => {
                let message = match panic.downcast_ref::<&'static str>() {
                    Some(msg) => *msg,
                    None => match panic.downcast_ref::<String>() {
                        Some(msg) => msg.as_str(),
                        None => "unknown panic",
                    },
                }
                .to_string();
                Err(BackendError {
                    kind: backend_error::Kind::AnkidroidPanicError as i32,
                    message,
                    ..Default::default()
                }
                .encode_to_vec())
            }
        }
    }
}

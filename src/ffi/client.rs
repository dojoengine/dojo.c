#[diplomat::bridge]
pub mod ffi {
    use diplomat_runtime::{DiplomatStr, DiplomatWrite};
    use lazy_static::lazy_static;
    use std::fmt::Write;
    use std::sync::Arc;
    use tokio::runtime::Runtime;
    use torii_client::Client as TClient;

    use crate::ffi::error::ffi::{DojoError, ErrorType};

    lazy_static! {
        static ref RUNTIME: Arc<Runtime> =
            Arc::new(Runtime::new().expect("Failed to create Tokio runtime"));
    }

    /// Opaque handle to a Torii client instance
    #[diplomat::opaque]
    pub struct ToriiClient {
        pub(crate) inner: Arc<TClient>,
    }

    impl ToriiClient {
        /// Creates a new Torii client instance
        pub fn new(torii_url: &DiplomatStr) -> Result<Box<ToriiClient>, Box<DojoError>> {
            let url = std::str::from_utf8(torii_url)?.to_string();
            
            let client = RUNTIME.block_on(TClient::new(url))
                .map_err(|e| DojoError::new(ErrorType::ClientError, &format!("Failed to create client: {}", e)))?;
            
            Ok(Box::new(ToriiClient {
                inner: Arc::new(client),
            }))
        }

        /// Gets information about the Torii server
        pub fn info(&self, result: &mut DiplomatWrite) -> Result<(), Box<DojoError>> {
            write!(result, "Torii client connected").unwrap();
            Ok(())
        }

        /// Publishes a message to the network
        pub fn publish_message(
            &self,
            message_json: &DiplomatStr,
            signature_r: &crate::ffi::types::ffi::FieldElement,
            signature_s: &crate::ffi::types::ffi::FieldElement,
            world_address: &crate::ffi::types::ffi::FieldElement,
            result: &mut DiplomatWrite,
        ) -> Result<(), Box<DojoError>> {
            use torii_proto::Message;
            
            let message_str = std::str::from_utf8(message_json)?.to_string();
            let message = Message {
                message: message_str,
                signature: vec![signature_r.0, signature_s.0],
                world_address: world_address.0,
            };
            
            let response = RUNTIME.block_on(self.inner.publish_message(message))
                .map_err(|e| DojoError::new(ErrorType::ClientError, &format!("Failed to publish message: {}", e)))?;
            
            write!(result, "{}", response).unwrap();
            Ok(())
        }
    }
}


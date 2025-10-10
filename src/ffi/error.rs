#[diplomat::bridge]
pub mod ffi {
    use diplomat_runtime::DiplomatWrite;
    use std::fmt::Write;

    /// Error types for Dojo operations
    #[diplomat::opaque]
    pub struct DojoError {
        pub(crate) error_type: ErrorType,
        pub(crate) message: String,
    }

    pub enum ErrorType {
        ClientError,
        ParseError,
        EntityError,
        QueryError,
        SubscriptionError,
        TransactionError,
        AccountError,
        SigningError,
        ProviderError,
        StorageError,
        ControllerError,
        InvalidInput,
        RuntimeError,
    }

    impl DojoError {
        pub fn new(error_type: ErrorType, message: &str) -> Box<DojoError> {
            Box::new(DojoError {
                error_type,
                message: message.to_string(),
            })
        }

        /// Gets the error message
        pub fn message(&self, result: &mut DiplomatWrite) {
            write!(result, "{}", self.message).unwrap();
        }

        /// Gets the error type
        pub fn error_type(&self) -> ErrorType {
            self.error_type
        }
    }

    // Implement From traits for common error types
    impl From<anyhow::Error> for Box<DojoError> {
        fn from(e: anyhow::Error) -> Self {
            Box::new(DojoError {
                error_type: ErrorType::RuntimeError,
                message: e.to_string(),
            })
        }
    }

    impl From<std::str::Utf8Error> for Box<DojoError> {
        fn from(e: std::str::Utf8Error) -> Self {
            Box::new(DojoError {
                error_type: ErrorType::InvalidInput,
                message: format!("Invalid UTF-8: {}", e),
            })
        }
    }

    impl From<url::ParseError> for Box<DojoError> {
        fn from(e: url::ParseError) -> Self {
            Box::new(DojoError {
                error_type: ErrorType::InvalidInput,
                message: format!("Invalid URL: {}", e),
            })
        }
    }

    impl From<starknet::core::types::FromStrError> for Box<DojoError> {
        fn from(e: starknet::core::types::FromStrError) -> Self {
            Box::new(DojoError {
                error_type: ErrorType::ParseError,
                message: format!("Parse error: {}", e),
            })
        }
    }

    impl From<serde_json::Error> for Box<DojoError> {
        fn from(e: serde_json::Error) -> Self {
            Box::new(DojoError {
                error_type: ErrorType::ParseError,
                message: format!("JSON parse error: {}", e),
            })
        }
    }

    impl From<torii_client::error::Error> for Box<DojoError> {
        fn from(e: torii_client::error::Error) -> Self {
            Box::new(DojoError {
                error_type: ErrorType::ClientError,
                message: format!("Torii client error: {:?}", e),
            })
        }
    }

    impl From<starknet::accounts::AccountError<starknet::providers::jsonrpc::HttpTransport>>
        for Box<DojoError>
    {
        fn from(
            e: starknet::accounts::AccountError<starknet::providers::jsonrpc::HttpTransport>,
        ) -> Self {
            Box::new(DojoError {
                error_type: ErrorType::AccountError,
                message: format!("Starknet account error: {:?}", e),
            })
        }
    }

    impl From<starknet::signers::local_wallet::SignError> for Box<DojoError> {
        fn from(e: starknet::signers::local_wallet::SignError) -> Self {
            Box::new(DojoError {
                error_type: ErrorType::SigningError,
                message: format!("Starknet signing error: {:?}", e),
            })
        }
    }
}


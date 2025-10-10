#[diplomat::bridge]
pub mod ffi {
    use diplomat_runtime::{DiplomatStr, DiplomatWrite};
    use lazy_static::lazy_static;
    use std::fmt::Write;
    use std::sync::Arc;
    use starknet::accounts::{Account as StarknetAccount, ConnectedAccount, SingleOwnerAccount, ExecutionEncoding};
    use starknet::providers::jsonrpc::HttpTransport;
    use starknet::providers::{JsonRpcClient, Provider as _};
    use starknet::signers::LocalWallet;
    use starknet_crypto::Felt;
    use tokio::runtime::Runtime;
    use url::Url;

    use crate::ffi::crypto::ffi::SigningKey;
    use crate::ffi::error::ffi::{DojoError, ErrorType};
    use crate::ffi::types::ffi::{CallList, FieldElement};

    lazy_static! {
        static ref RUNTIME: Arc<Runtime> =
            Arc::new(Runtime::new().expect("Failed to create Tokio runtime"));
    }

    /// JSON-RPC provider for Starknet
    #[diplomat::opaque]
    pub struct Provider {
        pub(crate) inner: Arc<JsonRpcClient<HttpTransport>>,
    }

    impl Provider {
        /// Creates a new provider from an RPC URL
        pub fn new(rpc_url: &DiplomatStr) -> Result<Box<Provider>, Box<DojoError>> {
            let url_str = std::str::from_utf8(rpc_url)?;
            let url = Url::parse(url_str)?;
            let provider = JsonRpcClient::new(HttpTransport::new(url));
            
            Ok(Box::new(Provider {
                inner: Arc::new(provider),
            }))
        }

        /// Gets the chain ID
        pub fn chain_id(&self, result: &mut DiplomatWrite) -> Result<(), Box<DojoError>> {
            let chain_id = RUNTIME.block_on(self.inner.chain_id())
                .map_err(|e| DojoError::new(ErrorType::ProviderError, &format!("Failed to get chain ID: {}", e)))?;
            
            write!(result, "{:#x}", chain_id).unwrap();
            Ok(())
        }

        /// Gets the latest block number
        pub fn block_number(&self) -> Result<u64, Box<DojoError>> {
            RUNTIME.block_on(self.inner.block_number())
                .map_err(|e| DojoError::new(ErrorType::ProviderError, &format!("Failed to get block number: {}", e)))
        }
    }

    /// Starknet account for signing and executing transactions
    #[diplomat::opaque]
    pub struct Account {
        pub(crate) inner: SingleOwnerAccount<Arc<JsonRpcClient<HttpTransport>>, LocalWallet>,
    }

    impl Account {
        /// Creates a new account
        pub fn new(
            provider: &Provider,
            signer: &SigningKey,
            address: &FieldElement,
            chain_id: &FieldElement,
        ) -> Box<Account> {
            let wallet = LocalWallet::from_signing_key(signer.0.clone());
            let account = SingleOwnerAccount::new(
                provider.inner.clone(),
                wallet,
                address.0,
                chain_id.0,
                ExecutionEncoding::New,
            );
            
            Box::new(Account { inner: account })
        }

        /// Gets the account address
        pub fn address(&self, result: &mut DiplomatWrite) {
            write!(result, "{:#x}", self.inner.address()).unwrap();
        }

        /// Gets the chain ID
        pub fn chain_id(&self, result: &mut DiplomatWrite) {
            write!(result, "{:#x}", self.inner.chain_id()).unwrap();
        }

        /// Executes a transaction with the given calls
        pub fn execute(
            &self,
            calls: &CallList,
            result: &mut DiplomatWrite,
        ) -> Result<(), Box<DojoError>> {
            let execution = self.inner.execute_v3(calls.calls.clone());
            
            let tx_result = RUNTIME.block_on(execution.send())
                .map_err(|e| DojoError::new(ErrorType::TransactionError, &format!("Failed to execute: {}", e)))?;
            
            write!(result, "{:#x}", tx_result.transaction_hash).unwrap();
            Ok(())
        }

        /// Gets the nonce for the account
        pub fn nonce(&self) -> Result<u64, Box<DojoError>> {
            let nonce = RUNTIME.block_on(self.inner.get_nonce())
                .map_err(|e| DojoError::new(ErrorType::AccountError, &format!("Failed to get nonce: {}", e)))?;
            
            // Convert Felt to u64 - this may truncate for large values
            Ok(nonce.to_string().parse().unwrap_or(0))
        }
    }

    /// Computes the contract address from class hash, salt, and constructor calldata
    pub fn compute_contract_address(
        deployer_address: &FieldElement,
        salt: &FieldElement,
        class_hash: &FieldElement,
        constructor_calldata: &[Box<FieldElement>],
    ) -> Box<FieldElement> {
        let calldata: Vec<Felt> = constructor_calldata.iter().map(|f| f.0).collect();
        let address = starknet::core::utils::get_contract_address(
            salt.0,
            class_hash.0,
            &calldata,
            deployer_address.0,
        );
        
        Box::new(FieldElement(address))
    }
}


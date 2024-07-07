use anyhow::Result;

use starknet::{
    core::types::StarknetError,
    providers::{Provider, ProviderError},
};
use starknet_crypto::Felt;

pub async fn watch_tx<P>(provider: P, transaction_hash: Felt) -> Result<()>
where
    P: Provider,
    ProviderError: 'static,
{
    loop {
        // TODO: check with sequencer gateway if it's not confirmed after an extended period of
        // time, as full nodes don't have access to failed transactions and would report them
        // as `NotReceived`.
        match provider.get_transaction_receipt(transaction_hash).await {
            Ok(_) => {
                // With JSON-RPC, once we get a receipt, the transaction must have been confirmed.
                // Rejected transactions simply aren't available. This needs to be changed once we
                // implement the sequencer fallback.

                return Ok(());
            }
            Err(ProviderError::StarknetError(StarknetError::TransactionHashNotFound)) => {}
            Err(err) => return Err(err.into()),
        }
    }
}

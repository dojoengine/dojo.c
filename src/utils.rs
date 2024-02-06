use anyhow::Result;

use starknet::{
    core::types::StarknetError,
    providers::{Provider, ProviderError},
};
use starknet_crypto::FieldElement;
use std::time::{Duration, Instant};
use tokio::time::sleep;

pub async fn watch_tx<P>(provider: P, transaction_hash: FieldElement) -> Result<()>
where
    P: Provider + 'static,
{
    loop {
        if start.elapsed() > timeout {
            // If the transaction has not been confirmed within the timeout,
            // query the sequencer gateway directly.
            // This part assumes you have a function `query_sequencer_gateway` to implement the direct check.
            // You may need to adjust it based on how you can actually query the sequencer.
            let status = query_sequencer_gateway(&transaction_hash).await?;
            match status {
                TransactionStatus::Confirmed => return Ok(()),
                TransactionStatus::Rejected => return Err(anyhow!("Transaction was rejected.")),
                TransactionStatus::Pending => {
                    return Err(anyhow!("Transaction is still pending after timeout."))
                }
                TransactionStatus::NotReceived => {
                    return Err(anyhow!("Transaction not received by the sequencer."))
                }
            }
        }

        match provider.get_transaction_receipt(transaction_hash).await {
            Ok(_) => {
                // Transaction is confirmed

                return Ok(());
            }
            Err(ProviderError::StarknetError(StarknetError::TransactionHashNotFound)) => {
                // Transaction not found, wait for the next interval
                sleep(check_interval).await;
            }
            Err(err) => return Err(err.into()),
        }
    }
}
async fn query_sequencer_gateway(transaction_hash: &FieldElement) -> Result<TransactionStatus> {
    // Implement querying the sequencer gateway here and return the transaction status.
    // This function is a placeholder and needs to be implemented based on your sequencer's API.
    Err(anyhow!("Sequencer gateway querying not implemented"))
}

enum TransactionStatus {
    Confirmed,
    Rejected,
    Pending,
    NotReceived,
}

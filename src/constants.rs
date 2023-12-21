use starknet::macros::felt;
use starknet_crypto::FieldElement;

pub const PREFUND_AMOUNT: FieldElement = felt!("0x2386f26fc10000"); // 0.001 ETH
pub const KATANA_ETH_CONTRACT_ADDRESS: FieldElement = felt!("0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7");
pub const KATANA_ACCOUNT_CLASS_HASH: FieldElement = felt!("0x04d07e40e93398ed3c76981e72dd1fd22557a78ce36c0515f679e27f0bb5bc5f");
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;

const URL: &str = "https://api.devnet.solana.com";

pub fn get_devnet_client() -> RpcClient {
    RpcClient::new_with_commitment(URL.to_string(), CommitmentConfig::confirmed())
}
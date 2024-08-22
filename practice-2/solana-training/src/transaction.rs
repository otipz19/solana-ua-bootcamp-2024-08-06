use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction,
    transaction::Transaction,
    signature::Keypair,
    signer::Signer,
};

pub async fn build_and_send_transaction(
    instructions: &[Instruction],
    payer: &Keypair,
    signers: &[&Keypair],
    client: &RpcClient
) -> Result<solana_sdk::signature::Signature, solana_client::client_error::ClientError> {
    let mut transaction = Transaction::new_with_payer(instructions, Some(&payer.pubkey()));
    let recent_blockhash = client.get_latest_blockhash().await.unwrap();
    transaction.sign(signers, recent_blockhash);
    client.send_and_confirm_transaction(&transaction).await
}

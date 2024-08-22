use std::str::FromStr;

use solana_sdk::{
    instruction::{ AccountMeta, Instruction },
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signer::Signer,
    system_instruction,
};
use crate::{
    connection::get_devnet_client,
    explorer::{ get_explorer_link, LinkType },
    keypair::get_keypair_from_env,
    transaction::build_and_send_transaction,
};

pub async fn send_sol() {
    let sender = get_keypair_from_env();
    let recipient = Pubkey::from_str("yqqJ3CEu7zikT8Dpks7ueKuY7qAF8uDN1esc15MKTvK").unwrap();

    let client = get_devnet_client();

    let transfer_instruction = system_instruction::transfer(
        &sender.pubkey(),
        &recipient,
        (0.1_f64 * (LAMPORTS_PER_SOL as f64)) as u64
    );
    let memo_instruction = Instruction::new_with_bytes(
        Pubkey::from_str("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr").unwrap(),
        &"This is my test transaction!".to_string().as_bytes(),
        vec![AccountMeta::new(sender.pubkey(), true)]
    );

    let signature = build_and_send_transaction(
        &[transfer_instruction, memo_instruction],
        &sender,
        &[&sender],
        &client
    ).await.unwrap();

    println!(
        "Transaction sent! Link: {}",
        get_explorer_link(LinkType::Transaction, &signature.to_string())
    );
}

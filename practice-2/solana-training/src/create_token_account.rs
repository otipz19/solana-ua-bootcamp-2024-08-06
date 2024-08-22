use std::str::FromStr;

use solana_sdk::{
    feature_set::spl_associated_token_account_v1_1_0,
    instruction::{ AccountMeta, Instruction },
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
};

use spl_associated_token_account::instruction::create_associated_token_account;

use crate::{
    connection::get_devnet_client, explorer::get_explorer_link, keypair::get_keypair_from_env, transaction::build_and_send_transaction
};

pub async fn create_token_account() {
    let sender = get_keypair_from_env();

    let receiver = Pubkey::from_str("yqqJ3CEu7zikT8Dpks7ueKuY7qAF8uDN1esc15MKTvK").unwrap();
    let mint = Pubkey::from_str("4mikvynVbGxxf7erx8KEbdLL65sZmcCaWEhyHDPQFfBz").unwrap();
    let token_program = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();

    let client = get_devnet_client();

    let create_token_account_instruction = create_associated_token_account(
        &sender.pubkey(),
        &receiver,
        &mint,
        &token_program
    );

    let signature = build_and_send_transaction(
        &[create_token_account_instruction],
        &sender,
        &[&sender],
        &client
    ).await.unwrap();

    println!("Created token account: {}", get_explorer_link(crate::explorer::LinkType::Transaction, &signature.to_string()));
}

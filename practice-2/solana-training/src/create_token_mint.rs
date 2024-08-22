use std::str::FromStr;

use solana_sdk::{
    native_token::sol_to_lamports, pubkey::Pubkey, signature::Keypair, signer::Signer, system_instruction
};
use crate::{
    connection::get_devnet_client, explorer::{self, get_explorer_link}, keypair::get_keypair_from_env, transaction::build_and_send_transaction
};
use spl_token::instruction::initialize_mint2;

pub async fn create_token_mint() {
    let sender = get_keypair_from_env();
    let token_program = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();
    let mint = Keypair::new();

    let create_account_instruction = system_instruction::create_account(
        &sender.pubkey(),
        &mint.pubkey(),
        sol_to_lamports(0.1),
        82,
        &token_program
    );

    let initialize_mint_instruction = initialize_mint2(
        &token_program,
        &mint.pubkey(),
        &sender.pubkey(),
        Option::None,
        2
    ).unwrap();

    let client = get_devnet_client();

    let signature = build_and_send_transaction(
        &[create_account_instruction, initialize_mint_instruction],
        &sender,
        &[&sender, &mint],
        &client
    ).await.unwrap();

    println!("Mint created!. Link: {}", get_explorer_link(explorer::LinkType::Transaction, &signature.to_string()));
}
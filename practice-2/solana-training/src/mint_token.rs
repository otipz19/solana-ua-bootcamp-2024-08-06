use std::str::FromStr;

use solana_sdk::{ pubkey::Pubkey, signer::Signer };

use crate::{
    connection::get_devnet_client, explorer::get_explorer_link, keypair::get_keypair_from_env, transaction::build_and_send_transaction
};

pub async fn mint_token() {
    let sender = get_keypair_from_env();

    let token_program = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();
    let mint = Pubkey::from_str("4mikvynVbGxxf7erx8KEbdLL65sZmcCaWEhyHDPQFfBz").unwrap();
    let token_account = Pubkey::from_str("AxM12pbJ9iqPsZwTRvF5UqAvL5Vf4FF9pHqtD3BCurE2").unwrap();

    let mint_to_instrutction = spl_token::instruction::mint_to(
        &token_program,
        &mint,
        &token_account,
        &sender.pubkey(),
        &[&sender.pubkey()],
        69 * 100
    ).unwrap();

    let client = get_devnet_client();
    let signature = build_and_send_transaction(
        &[mint_to_instrutction],
        &sender,
        &[&sender],
        &client
    ).await.unwrap();

    println!("Minted token: {}", get_explorer_link(crate::explorer::LinkType::Transaction, &signature.to_string()));
}

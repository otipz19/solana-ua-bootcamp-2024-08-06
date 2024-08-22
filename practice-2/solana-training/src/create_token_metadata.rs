use std::str::FromStr;

use mpl_token_metadata::{
    instructions::{ CreateMetadataAccountV3, CreateMetadataAccountV3InstructionArgs },
    types::DataV2,
};
use solana_program::pubkey::Pubkey;
use solana_sdk::{pubkey::Pubkey as PubkeySdk, signer::Signer, signature::Keypair, signer::keypair, transaction::Transaction};

use crate::{ connection::get_devnet_client, keypair::get_keypair_from_env, transaction::build_and_send_transaction };

pub async fn create_token_metadata() {
    let sender = get_keypair_from_env();
    let sender_pubkey = get_pubkey_from_keypair(&sender);

    let token_metadata_program = solana_program::pubkey::Pubkey::from_str(
        "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
    ).unwrap();
    let receiver = solana_program::pubkey::Pubkey::from_str("yqqJ3CEu7zikT8Dpks7ueKuY7qAF8uDN1esc15MKTvK").unwrap();
    let mint = solana_program::pubkey::Pubkey::from_str("4mikvynVbGxxf7erx8KEbdLL65sZmcCaWEhyHDPQFfBz").unwrap();

    let (metadata_pda, bump) = solana_program::pubkey::Pubkey::find_program_address(
        &["metadata".to_string().as_bytes(), &token_metadata_program.to_bytes(), &mint.to_bytes()],
        &token_metadata_program
    );

    let create_metadata_instruction = (CreateMetadataAccountV3 {
        metadata: metadata_pda,
        mint,
        mint_authority: sender_pubkey,
        update_authority: (sender_pubkey, true),
        payer: sender_pubkey,
        rent: Option::None,
        system_program: solana_program::pubkey::Pubkey::from_str("11111111111111111111111111111111").unwrap(),
    }).instruction(CreateMetadataAccountV3InstructionArgs {
        is_mutable: true,
        collection_details: Option::None,
        data: DataV2 {
            name: "My Rust Token!".to_string(),
            symbol: "MRT".to_string(),
            uri: "https://arweave.net/1234".to_string(),
            creators: Option::None,
            collection: Option::None,
            uses: Option::None,
            seller_fee_basis_points: 0,
        },
    });

    // let signature = build_and_send_transaction(
    //     &[create_metadata_instruction],
    //     &sender,
    //     &[&sender],
    //     &get_devnet_client()
    // ).await.unwrap();

    let client = get_devnet_client();
    let mut transaction = Transaction::new_with_payer(&[solana_sdk::instruction::Instruction {
        program_id: create_metadata_instruction.program_id,
        data: create_metadata_instruction.data,
        accounts: create_metadata_instruction.accounts
    }], Some(&sender.pubkey()));
    let recent_blockhash = client.get_latest_blockhash().await.unwrap();
    transaction.sign(&[&sender], recent_blockhash);
    let signature = client.send_and_confirm_transaction(&transaction).await.unwrap();

    // solana_sdk::instruction::Instruction::

    println!("Created metadata: {}", &signature.to_string());
}

fn get_pubkey_from_keypair(keypair: &Keypair) -> solana_program::pubkey::Pubkey {
    solana_program::pubkey::Pubkey::from_str(&keypair.pubkey().to_string()).unwrap()
}

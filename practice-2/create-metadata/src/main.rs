use std::str::FromStr;
use solana_sdk::{ signature::Keypair, signer::{keypair, Signer}, commitment_config::CommitmentConfig, transaction::Transaction };
use mpl_token_metadata::{instructions::{CreateMetadataAccountV3, CreateMetadataAccountV3InstructionArgs}, types::DataV2};
use solana_client::nonblocking::rpc_client::RpcClient;

#[tokio::main]
async fn main() {
    let sender = get_keypair_from_env();

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
        mint_authority: solana_program::pubkey::Pubkey::from_str(&sender.pubkey().to_string()).unwrap(),
        update_authority: (solana_program::pubkey::Pubkey::from_str(&sender.pubkey().to_string()).unwrap(), true),
        payer: solana_program::pubkey::Pubkey::from_str(&sender.pubkey().to_string()).unwrap(),
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

    let client = get_devnet_client();
    let mut transaction = Transaction::new_with_payer(&[solana_sdk::instruction::Instruction {
        program_id: create_metadata_instruction.program_id,
        data: create_metadata_instruction.data,
        accounts: create_metadata_instruction.accounts
    }], Some(&sender.pubkey()));
    let recent_blockhash = client.get_latest_blockhash().await.unwrap();
    transaction.sign(&[&sender], recent_blockhash);
    let signature = client.send_and_confirm_transaction(&transaction).await.unwrap();

    println!("Created metadata: {}", &signature.to_string());
}

fn get_keypair_from_env() -> Keypair {
    dotenv::dotenv().ok();
    let secret_key = std::env::var("SECRET_KEY").unwrap();
    let vec: Vec<u8> = serde_json::from_str(&secret_key).unwrap();
    let bytes: [u8; 64] = vec.try_into().unwrap();
    keypair::Keypair::from_bytes(&bytes).unwrap()
}

fn get_pubkey_from_keypair(keypair: &Keypair) -> solana_program::pubkey::Pubkey {
    solana_program::pubkey::Pubkey::from_str(&keypair.pubkey().to_string()).unwrap()
}

const URL: &str = "https://api.devnet.solana.com";

pub fn get_devnet_client() -> RpcClient {
    RpcClient::new_with_commitment(URL.to_string(), CommitmentConfig::confirmed())
}
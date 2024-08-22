use solana_sdk::{ signature::Keypair, signer::keypair };

pub fn get_keypair_from_env() -> Keypair {
    dotenv::dotenv().ok();
    let secret_key = std::env::var("SECRET_KEY").unwrap();
    let vec: Vec<u8> = serde_json::from_str(&secret_key).unwrap();
    let bytes: [u8; 64] = vec.try_into().unwrap();
    keypair::Keypair::from_bytes(&bytes).unwrap()
}
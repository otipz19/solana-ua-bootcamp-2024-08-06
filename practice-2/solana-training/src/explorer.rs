pub fn get_explorer_link(link_type: LinkType, id: &str) -> String {
    format!(
        "https://explorer.solana.com/{}/{}?cluster=devnet",
        match link_type {
            LinkType::Address => "address",
            LinkType::Transaction => "tx",
        },
        id
    )
}

pub enum LinkType {
    Address,
    Transaction,
}

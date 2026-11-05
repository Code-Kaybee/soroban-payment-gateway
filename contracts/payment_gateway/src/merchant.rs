use soroban_sdk::{contracttype, Address, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Merchant {
    pub id: String,
    pub address: Address,
    pub name: String,
    pub active: bool,
    pub total_volume: i128,
    pub transaction_count: u64,
}

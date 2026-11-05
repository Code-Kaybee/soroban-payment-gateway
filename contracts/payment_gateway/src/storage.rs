use soroban_sdk::{contracttype, Address, Env, String};

use crate::{Escrow, Merchant, Payment};

#[contracttype]
#[derive(Clone)]
enum DataKey {
    Admin,
    PlatformFee,
    PaymentCounter,
    EscrowCounter,
    Merchant(String),
    Payment(u64),
    Escrow(u64),
}

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}

pub fn get_admin(env: &Env) -> Address {
    env.storage()
        .instance()
        .get(&DataKey::Admin)
        .unwrap_or_else(|| panic!("Admin not set"))
}

pub fn has_admin(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Admin)
}

pub fn set_platform_fee(env: &Env, fee: u32) {
    env.storage().instance().set(&DataKey::PlatformFee, &fee);
}

pub fn get_platform_fee(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&DataKey::PlatformFee)
        .unwrap_or(0)
}

pub fn set_merchant(env: &Env, merchant_id: &String, merchant: &Merchant) {
    env.storage()
        .persistent()
        .set(&DataKey::Merchant(merchant_id.clone()), merchant);
}

pub fn get_merchant(env: &Env, merchant_id: &String) -> Option<Merchant> {
    env.storage()
        .persistent()
        .get(&DataKey::Merchant(merchant_id.clone()))
}

pub fn has_merchant(env: &Env, merchant_id: &String) -> bool {
    env.storage()
        .persistent()
        .has(&DataKey::Merchant(merchant_id.clone()))
}

pub fn get_next_payment_id(env: &Env) -> u64 {
    let current: u64 = env
        .storage()
        .instance()
        .get(&DataKey::PaymentCounter)
        .unwrap_or(0);
    let next = current + 1;
    env.storage()
        .instance()
        .set(&DataKey::PaymentCounter, &next);
    next
}

pub fn set_payment(env: &Env, payment_id: u64, payment: &Payment) {
    env.storage()
        .persistent()
        .set(&DataKey::Payment(payment_id), payment);
}

pub fn get_payment(env: &Env, payment_id: u64) -> Option<Payment> {
    env.storage()
        .persistent()
        .get(&DataKey::Payment(payment_id))
}

pub fn get_next_escrow_id(env: &Env) -> u64 {
    let current: u64 = env
        .storage()
        .instance()
        .get(&DataKey::EscrowCounter)
        .unwrap_or(0);
    let next = current + 1;
    env.storage().instance().set(&DataKey::EscrowCounter, &next);
    next
}

pub fn set_escrow(env: &Env, escrow_id: u64, escrow: &Escrow) {
    env.storage()
        .persistent()
        .set(&DataKey::Escrow(escrow_id), escrow);
}

pub fn get_escrow(env: &Env, escrow_id: u64) -> Option<Escrow> {
    env.storage().persistent().get(&DataKey::Escrow(escrow_id))
}

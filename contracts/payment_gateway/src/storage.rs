use soroban_sdk::{Address, Env, String};

use crate::{Escrow, Merchant, Payment};

// Storage keys
const ADMIN_KEY: &str = "Admin";
const PLATFORM_FEE_KEY: &str = "PlatformFee";
const PAYMENT_COUNTER_KEY: &str = "PaymentCounter";
const ESCROW_COUNTER_KEY: &str = "EscrowCounter";

// Admin functions
pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&ADMIN_KEY, admin);
}

pub fn get_admin(env: &Env) -> Address {
    env.storage()
        .instance()
        .get(&ADMIN_KEY)
        .unwrap_or_else(|| panic!("Admin not set"))
}

pub fn has_admin(env: &Env) -> bool {
    env.storage().instance().has(&ADMIN_KEY)
}

// Platform fee functions
pub fn set_platform_fee(env: &Env, fee: u32) {
    env.storage().instance().set(&PLATFORM_FEE_KEY, &fee);
}

pub fn get_platform_fee(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&PLATFORM_FEE_KEY)
        .unwrap_or(0)
}

// Merchant functions
pub fn set_merchant(env: &Env, merchant_id: &String, merchant: &Merchant) {
    let key = format!("Merchant:{}", merchant_id);
    env.storage().persistent().set(&key, merchant);
}

pub fn get_merchant(env: &Env, merchant_id: &String) -> Option<Merchant> {
    let key = format!("Merchant:{}", merchant_id);
    env.storage().persistent().get(&key)
}

pub fn has_merchant(env: &Env, merchant_id: &String) -> bool {
    let key = format!("Merchant:{}", merchant_id);
    env.storage().persistent().has(&key)
}

// Payment functions
pub fn get_next_payment_id(env: &Env) -> u64 {
    let current: u64 = env
        .storage()
        .instance()
        .get(&PAYMENT_COUNTER_KEY)
        .unwrap_or(0);
    let next = current + 1;
    env.storage().instance().set(&PAYMENT_COUNTER_KEY, &next);
    next
}

pub fn set_payment(env: &Env, payment_id: u64, payment: &Payment) {
    let key = format!("Payment:{}", payment_id);
    env.storage().persistent().set(&key, payment);
}

pub fn get_payment(env: &Env, payment_id: u64) -> Option<Payment> {
    let key = format!("Payment:{}", payment_id);
    env.storage().persistent().get(&key)
}

// Escrow functions
pub fn get_next_escrow_id(env: &Env) -> u64 {
    let current: u64 = env
        .storage()
        .instance()
        .get(&ESCROW_COUNTER_KEY)
        .unwrap_or(0);
    let next = current + 1;
    env.storage().instance().set(&ESCROW_COUNTER_KEY, &next);
    next
}

pub fn set_escrow(env: &Env, escrow_id: u64, escrow: &Escrow) {
    let key = format!("Escrow:{}", escrow_id);
    env.storage().persistent().set(&key, escrow);
}

pub fn get_escrow(env: &Env, escrow_id: u64) -> Option<Escrow> {
    let key = format!("Escrow:{}", escrow_id);
    env.storage().persistent().get(&key)
}

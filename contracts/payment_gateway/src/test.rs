use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token, Address, Env, String,
};

fn setup() -> (Env, Address, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_address = env
        .register_stellar_asset_contract_v2(admin.clone())
        .address();
    let contract_id = env.register_contract(None, PaymentGateway);
    PaymentGatewayClient::new(&env, &contract_id).initialize(&admin, &250);

    (env, contract_id, admin, token_address)
}

fn register_merchant(
    client: &PaymentGatewayClient<'_>,
    env: &Env,
    merchant_id: &str,
    merchant_address: &Address,
) {
    client.register_merchant(
        &String::from_str(env, merchant_id),
        merchant_address,
        &String::from_str(env, "Test merchant"),
    );
}

#[test]
fn initializes_with_configured_platform_fee() {
    let (env, contract_id, _, _) = setup();
    let client = PaymentGatewayClient::new(&env, &contract_id);

    assert_eq!(client.get_platform_fee(), 250);
}

#[test]
#[should_panic(expected = "Platform fee cannot exceed 100%")]
fn rejects_platform_fee_above_100_percent() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let contract_id = env.register_contract(None, PaymentGateway);
    let client = PaymentGatewayClient::new(&env, &contract_id);

    client.initialize(&admin, &10_001);
}

#[test]
fn payment_splits_amount_and_records_payment_and_merchant_volume() {
    let (env, contract_id, admin, token_address) = setup();
    let client = PaymentGatewayClient::new(&env, &contract_id);
    let payer = Address::generate(&env);
    let merchant_address = Address::generate(&env);
    let merchant_id = String::from_str(&env, "merchant-1");
    let order_id = String::from_str(&env, "order-1");

    token::StellarAssetClient::new(&env, &token_address).mint(&payer, &10_000);
    register_merchant(&client, &env, "merchant-1", &merchant_address);

    let payment_id =
        client.process_payment(&payer, &merchant_id, &token_address, &10_000, &order_id);

    let token = token::Client::new(&env, &token_address);
    assert_eq!(token.balance(&payer), 0);
    assert_eq!(token.balance(&merchant_address), 9_750);
    assert_eq!(token.balance(&admin), 250);

    let payment = client.get_payment(&payment_id).unwrap();
    assert_eq!(payment.amount, 10_000);
    assert_eq!(payment.fee_amount, 250);
    assert_eq!(payment.order_id, order_id);
    assert_eq!(payment.status, PaymentStatus::Completed);

    let merchant = client.get_merchant(&merchant_id).unwrap();
    assert_eq!(merchant.total_volume, 10_000);
    assert_eq!(merchant.transaction_count, 1);
}

#[test]
fn buyer_can_release_escrow_before_release_time() {
    let (env, contract_id, _, token_address) = setup();
    let client = PaymentGatewayClient::new(&env, &contract_id);
    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);

    env.ledger().set_timestamp(100);
    token::StellarAssetClient::new(&env, &token_address).mint(&buyer, &500);
    let escrow_id = client.create_escrow(
        &buyer,
        &seller,
        &token_address,
        &500,
        &String::from_str(&env, "order-escrow"),
        &200,
    );

    assert_eq!(
        token::Client::new(&env, &token_address).balance(&contract_id),
        500
    );

    client.release_escrow(&escrow_id);

    assert_eq!(
        token::Client::new(&env, &token_address).balance(&seller),
        500
    );
    assert_eq!(
        client.get_escrow(&escrow_id).unwrap().status,
        EscrowStatus::Released
    );
}

#[test]
fn anyone_can_release_escrow_after_release_time() {
    let (env, contract_id, _, token_address) = setup();
    let client = PaymentGatewayClient::new(&env, &contract_id);
    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);

    env.ledger().set_timestamp(100);
    token::StellarAssetClient::new(&env, &token_address).mint(&buyer, &500);
    let escrow_id = client.create_escrow(
        &buyer,
        &seller,
        &token_address,
        &500,
        &String::from_str(&env, "order-auto-release"),
        &200,
    );

    env.ledger().set_timestamp(200);
    client.release_escrow(&escrow_id);

    assert_eq!(
        token::Client::new(&env, &token_address).balance(&seller),
        500
    );
    assert_eq!(
        client.get_escrow(&escrow_id).unwrap().status,
        EscrowStatus::Released
    );
}

#[test]
fn seller_can_refund_active_escrow_to_buyer() {
    let (env, contract_id, _, token_address) = setup();
    let client = PaymentGatewayClient::new(&env, &contract_id);
    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);

    env.ledger().set_timestamp(100);
    token::StellarAssetClient::new(&env, &token_address).mint(&buyer, &500);
    let escrow_id = client.create_escrow(
        &buyer,
        &seller,
        &token_address,
        &500,
        &String::from_str(&env, "order-refund"),
        &200,
    );

    client.refund_escrow(&escrow_id);

    assert_eq!(
        token::Client::new(&env, &token_address).balance(&buyer),
        500
    );
    assert_eq!(
        client.get_escrow(&escrow_id).unwrap().status,
        EscrowStatus::Refunded
    );
}

#[test]
fn fee_calculation_handles_maximum_i128_amount_without_overflow() {
    assert_eq!(calculate_fee(i128::MAX, 10_000), i128::MAX);
    assert_eq!(calculate_fee(i128::MAX, 0), 0);
    assert_eq!(calculate_fee(399, 250), 9);
    assert_eq!(calculate_fee(400, 250), 10);
}

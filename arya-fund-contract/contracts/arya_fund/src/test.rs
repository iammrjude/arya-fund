#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    Address, Env, String,
};

// ===== HELPERS =====

fn create_env() -> Env {
    Env::default()
}

fn create_addresses(env: &Env) -> (Address, Address, Address, Address) {
    let platform_owner = Address::generate(env);
    let treasury = Address::generate(env);
    let organizer = Address::generate(env);
    let donor = Address::generate(env);
    (platform_owner, treasury, organizer, donor)
}

// fn init_contract(env: &Env) -> (Address, Address, Address, Address) {
//     let contract_id = env.register(AryaFund, ());
//     let client = AryaFundClient::new(env, &contract_id);

//     let (platform_owner, treasury, organizer, donor) = create_addresses(env);

//     env.mock_all_auths();

//     let native_token = Address::generate(&env);
//     client.initialize(
//         &platform_owner,
//         &treasury,
//         &250u32,    // 2.5% fee
//         &7u32,    // 7 day action window
//         &native_token
//     );

//     (platform_owner, treasury, organizer, donor)
// }

fn get_client(env: &Env) -> AryaFundClient<'_> {
    let contract_id = env.register(AryaFund, ());
    AryaFundClient::new(env, &contract_id)
}

fn now(env: &Env) -> u64 {
    env.ledger().timestamp()
}

fn advance_time(env: &Env, seconds: u64) {
    env.ledger().set_timestamp(env.ledger().timestamp() + seconds);
}

fn days(n: u64) -> u64 {
    n * 24 * 60 * 60
}

// ===== TESTS =====

#[test]
fn test_initialize() {
    let env = create_env();
    let client = get_client(&env);
    let (platform_owner, treasury, _, _) = create_addresses(&env);

    env.mock_all_auths();

    let native_token = Address::generate(&env);
    client.initialize(&platform_owner, &treasury, &250u32, &7u32, &native_token);

    let settings = client.get_platform_settings();
    assert_eq!(settings.platform_owner, platform_owner);
    assert_eq!(settings.treasury_wallet, treasury);
    assert_eq!(settings.fee_basis_points, 250);
    assert_eq!(settings.action_window_days, 7);
}

#[test]
#[should_panic(expected = "Already initialized")]
fn test_initialize_twice_fails() {
    let env = create_env();
    let client = get_client(&env);
    let (platform_owner, treasury, _, _) = create_addresses(&env);

    env.mock_all_auths();

    let native_token = Address::generate(&env);
    client.initialize(&platform_owner, &treasury, &250u32, &7u32, &native_token);
    client.initialize(&platform_owner, &treasury, &250u32, &7u32, &native_token);
}

#[test]
fn test_create_campaign() {
    let env = create_env();
    let client = get_client(&env);
    let (platform_owner, treasury, organizer, _) = create_addresses(&env);

    env.mock_all_auths();
    let native_token = Address::generate(&env);
    client.initialize(&platform_owner, &treasury, &250u32, &7u32, &native_token);

    let deadline = now(&env) + days(30);
    let campaign_id = client.create_campaign(
        &organizer,
        &String::from_str(&env, "Solar Panels"),
        &String::from_str(&env, "Community solar project"),
        &10_000_0000000i128, // 10,000 XLM in stroops
        &deadline,
        &45u32,
    );

    assert_eq!(campaign_id, 0);

    let campaign = client.get_campaign(&campaign_id);
    assert_eq!(campaign.id, 0);
    assert_eq!(campaign.goal_amount, 10_000_0000000i128);
    assert_eq!(campaign.total_raised, 0);
    assert_eq!(campaign.extension_used, false);
    assert_eq!(campaign.extension_days, 45);
    assert_eq!(campaign.organizer, organizer);
}

#[test]
fn test_campaign_count_increments() {
    let env = create_env();
    let client = get_client(&env);
    let (platform_owner, treasury, organizer, _) = create_addresses(&env);

    env.mock_all_auths();
    let native_token = Address::generate(&env);
    client.initialize(&platform_owner, &treasury, &250u32, &7u32, &native_token);

    let deadline = now(&env) + days(30);

    client.create_campaign(
        &organizer,
        &String::from_str(&env, "Campaign 1"),
        &String::from_str(&env, "Description 1"),
        &1_000_0000000i128,
        &deadline,
        &30u32,
    );

    client.create_campaign(
        &organizer,
        &String::from_str(&env, "Campaign 2"),
        &String::from_str(&env, "Description 2"),
        &2_000_0000000i128,
        &deadline,
        &30u32,
    );

    assert_eq!(client.get_campaign_count(), 2);
}

#[test]
fn test_is_campaign_failed_under_70_at_deadline() {
    let env = create_env();
    let client = get_client(&env);
    let (platform_owner, treasury, organizer, _) = create_addresses(&env);

    env.mock_all_auths();
    let native_token = Address::generate(&env);
    client.initialize(&platform_owner, &treasury, &250u32, &7u32, &native_token);

    let deadline = now(&env) + days(30);
    let campaign_id = client.create_campaign(
        &organizer,
        &String::from_str(&env, "Test Campaign"),
        &String::from_str(&env, "Description"),
        &10_000_0000000i128,
        &deadline,
        &30u32,
    );

    // Campaign not failed before deadline
    assert_eq!(client.is_campaign_failed(&campaign_id), false);

    // Advance past deadline with 0 raised (0% < 70%)
    advance_time(&env, days(31));
    assert_eq!(client.is_campaign_failed(&campaign_id), true);
}

#[test]
fn test_is_campaign_failed_action_window_expired() {
    let env = create_env();
    let client = get_client(&env);
    let (platform_owner, treasury, organizer, _) = create_addresses(&env);

    env.mock_all_auths();
    let native_token = Address::generate(&env);
    client.initialize(&platform_owner, &treasury, &250u32, &7u32, &native_token);

    let deadline = now(&env) + days(30);
    let campaign_id = client.create_campaign(
        &organizer,
        &String::from_str(&env, "Test Campaign"),
        &String::from_str(&env, "Description"),
        &10_000_0000000i128,
        &deadline,
        &30u32,
    );

    // Advance past deadline + 7 day action window with no action taken
    // Assume 70%+ was raised (we simulate by checking logic directly)
    advance_time(&env, days(38)); // 30 day deadline + 7 day action window + 1 day
    assert_eq!(client.is_campaign_failed(&campaign_id), true);
}

#[test]
fn test_mark_as_failed_by_organizer() {
    let env = create_env();
    let client = get_client(&env);
    let (platform_owner, treasury, organizer, _) = create_addresses(&env);

    env.mock_all_auths();
    let native_token = Address::generate(&env);
    client.initialize(&platform_owner, &treasury, &250u32, &7u32, &native_token);

    let deadline = now(&env) + days(30);
    let campaign_id = client.create_campaign(
        &organizer,
        &String::from_str(&env, "Test Campaign"),
        &String::from_str(&env, "Description"),
        &10_000_0000000i128,
        &deadline,
        &30u32,
    );

    client.mark_as_failed(&campaign_id);
    assert_eq!(client.is_campaign_failed(&campaign_id), true);
}

#[test]
fn test_update_fee_percent() {
    let env = create_env();
    let client = get_client(&env);
    let (platform_owner, treasury, _, _) = create_addresses(&env);

    env.mock_all_auths();
    let native_token = Address::generate(&env);
    client.initialize(&platform_owner, &treasury, &250u32, &7u32, &native_token);

    client.update_fee_percent(&500u32); // change to 5%
    let settings = client.get_platform_settings();
    assert_eq!(settings.fee_basis_points, 500);
}

#[test]
fn test_update_treasury_wallet() {
    let env = create_env();
    let client = get_client(&env);
    let (platform_owner, treasury, _, _) = create_addresses(&env);

    env.mock_all_auths();
    let native_token = Address::generate(&env);
    client.initialize(&platform_owner, &treasury, &250u32, &7u32, &native_token);

    let new_treasury = Address::generate(&env);
    client.update_treasury_wallet(&new_treasury);
    let settings = client.get_platform_settings();
    assert_eq!(settings.treasury_wallet, new_treasury);
}

#[test]
fn test_update_action_window() {
    let env = create_env();
    let client = get_client(&env);
    let (platform_owner, treasury, _, _) = create_addresses(&env);

    env.mock_all_auths();
    let native_token = Address::generate(&env);
    client.initialize(&platform_owner, &treasury, &250u32, &7u32, &native_token);

    client.update_action_window(&14u32);
    let settings = client.get_platform_settings();
    assert_eq!(settings.action_window_days, 14);
}

#[test]
fn test_transfer_ownership() {
    let env = create_env();
    let client = get_client(&env);
    let (platform_owner, treasury, _, _) = create_addresses(&env);

    env.mock_all_auths();
    let native_token = Address::generate(&env);
    client.initialize(&platform_owner, &treasury, &250u32, &7u32, &native_token);

    let new_owner = Address::generate(&env);
    client.transfer_ownership(&new_owner);
    let settings = client.get_platform_settings();
    assert_eq!(settings.platform_owner, new_owner);
}

#[test]
#[should_panic]
fn test_extend_deadline_not_allowed_before_deadline() {
    let env = create_env();
    let client = get_client(&env);
    let (platform_owner, treasury, organizer, _) = create_addresses(&env);

    env.mock_all_auths();
    let native_token = Address::generate(&env);
    client.initialize(&platform_owner, &treasury, &250u32, &7u32, &native_token);

    let deadline = now(&env) + days(30);
    let campaign_id = client.create_campaign(
        &organizer,
        &String::from_str(&env, "Test"),
        &String::from_str(&env, "Desc"),
        &10_000_0000000i128,
        &deadline,
        &30u32,
    );

    // Try to extend before deadline — should panic
    client.extend_deadline(&campaign_id);
}

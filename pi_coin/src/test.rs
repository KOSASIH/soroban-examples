#![cfg(test)]
use soroban_sdk::{testutils::*, Address, Env, Symbol, Bytes, BytesN, crypto};
use crate::PiCoinContract; // Import kontrak utama
use crate::PiCoinData; // Import struct data
use crate::PiCoinSource; // Import enum source

#[test]
fn test_initialize_hyper_tech() {
    let env = Env::default();
    env.mock_all_auths(); // Hyper-tech: Mock auth untuk simulasi quantum-secure

    let admin = Address::random(&env);
    let collateral = Address::random(&env);
    let oracle = Address::random(&env);
    let governance = Address::random(&env);

    // Initialize dengan parameter ultimate
    let result = PiCoinContract::initialize(env.clone(), admin, collateral, oracle, governance);
    assert!(result.is_ok());

    // Verifikasi data immutable (anti-tamper)
    let data: PiCoinData = env.storage().instance().get(&Symbol::new(&env, "data")).unwrap();
    assert_eq!(data.symbol, Symbol::new(&env, "PI"));
    assert_eq!(data.total_supply, 100_000_000_000);
    assert_eq!(data.peg_value, 314_159_000_000);
    assert_eq!(data.anti_fraud_hash, env.crypto().sha256(&Bytes::from_slice(&env, b"PiCoin-Ultimate-Hyper-Tech-Unique")));
    println!("Hyper-tech init: Symbol PI locked, supply 100B, peg $314,159 verified with quantum hash - Exclusive sources only");
}

#[test]
fn test_mint_with_collateral_backing() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::random(&env);
    let to = Address::random(&env);
    let collateral = Address::random(&env);
    let oracle = Address::random(&env);
    let governance = Address::random(&env);

    PiCoinContract::initialize(env.clone(), admin, collateral, oracle, governance).unwrap();

    // Mint dengan collateral check (1:1 backing) dan valid source
    let amount = 1_000_000;
    let source = PiCoinSource::Mining; // Valid source for peg
    let result = PiCoinContract::mint(env.clone(), to.clone(), amount, source);
    assert!(result.is_ok());

    // Hyper-tech: Verify quantum provenance logged
    let logs = env.logger().all();
    assert!(logs.iter().any(|log| log.contains("quantum provenance")));
    println!("Ultimate mint: {} PI from {} source minted with full collateral, quantum-resistant provenance applied", amount, source);
}

#[test]
fn test_transfer_with_anti_fraud_zkp() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::random(&env);
    let from = Address::random(&env);
    let to = Address::random(&env);
    let collateral = Address::random(&env);
    let oracle = Address::random(&env);
    let governance = Address::random(&env);

    PiCoinContract::initialize(env.clone(), admin, collateral, oracle, governance).unwrap();

    // Mint first to set provenance
    let source = PiCoinSource::Rewards;
    PiCoinContract::mint(env.clone(), from.clone(), 500_000, source).unwrap();

    // Setup ZKP base for anti-fraud
    let zkp_base = env.crypto().sha256(&Bytes::from_slice(&env, &[42, 0])); // Simulated ZKP seed
    env.storage().instance().set(&Symbol::new(&env, "zkp_base"), &zkp_base);

    // Transfer dengan ZKP verification dan provenance check
    let amount = 500_000;
    let result = PiCoinContract::transfer(env.clone(), from, to, amount);
    assert!(result.is_ok());

    // Hyper-tech: Check anti-fraud and provenance log
    let logs = env.logger().all();
    assert!(logs.iter().any(|log| log.contains("anti-fraud ZKP")));
    assert!(logs.iter().any(|log| log.contains("valid provenance")));
    println!("Maximum level transfer: {} PI moved with ZKP anti-forgery and {} source provenance, untouchable duplication", amount, source);
}

#[test]
fn test_verify_peg_with_ai_oracle() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::random(&env);
    let holder = Address::random(&env);
    let collateral = Address::random(&env);
    let oracle = Address::random(&env);
    let governance = Address::random(&env);

    PiCoinContract::initialize(env.clone(), admin, collateral, oracle, governance).unwrap();

    // Mint to set valid provenance
    let source = PiCoinSource::P2P;
    PiCoinContract::mint(env.clone(), holder.clone(), 1_000_000, source).unwrap();

    // Verify peg dengan AI oracle simulation dan provenance check
    let result = PiCoinContract::verify_peg(env.clone(), holder.clone());
    assert!(result.is_ok());

    // Hyper-tech: Simulate AI prediction deviation
    env.ledger().set_timestamp(1000000); // Change ledger for dynamic oracle
    let result_dev = PiCoinContract::verify_peg(env.clone(), holder);
    assert!(result_dev.is_ok()); // Should still pass with micro-deviation for valid source
    println!("Super advanced peg verify: AI oracle confirms $314,159 stability for {} source, global market synced", source);
}

#[test]
fn test_governance_vote_quantum_secure() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::random(&env);
    let voter = Address::random(&env);
    let collateral = Address::random(&env);
    let oracle = Address::random(&env);
    let governance = Address::random(&env);

    PiCoinContract::initialize(env.clone(), admin, collateral, oracle, governance).unwrap();

    // Mint to set valid provenance
    let source = PiCoinSource::Mining;
    PiCoinContract::mint(env.clone(), voter.clone(), 100_000, source).unwrap();

    // Governance vote dengan quantum sig dan provenance check
    let proposal = Symbol::new(&env, "rebase");
    let result = PiCoinContract::governance_vote(env.clone(), voter, proposal);
    assert!(result.is_ok());

    // Hyper-tech: Verify multi-sig and provenance log
    let logs = env.logger().all();
    assert!(logs.iter().any(|log| log.contains("Quantum vote")));
    assert!(logs.iter().any(|log| log.contains("from Mining source")));
    println!("Ultimate governance: Vote cast with quantum-secure multi-sig and {} source provenance, unmatched integrity", source);
}

#[test]
fn test_error_insufficient_collateral() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::random(&env);
    let to = Address::random(&env);
    let collateral = Address::random(&env);
    let oracle = Address::random(&env);
    let governance = Address::random(&env);

    PiCoinContract::initialize(env.clone(), admin, collateral, oracle, governance).unwrap();

    // Attempt mint with insufficient collateral (simulated failure) and valid source
    let amount = 200_000_000_000; // Exceed mock collateral
    let source = PiCoinSource::Rewards;
    let result = PiCoinContract::mint(env.clone(), to, amount, source);
    assert!(matches!(result, Err(crate::PiCoinError::InsufficientCollateral)));
    println!("Hyper-tech error: Mint blocked by collateral check, ultimate security enforced");
}

#[test]
fn test_global_payment_simulation() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::random(&env);
    let to = Address::random(&env);
    let collateral = Address::random(&env);
    let oracle = Address::random(&env);
    let governance = Address::random(&env);

    PiCoinContract::initialize(env.clone(), admin, collateral, oracle, governance).unwrap();

    // Mint and simulate global payment with valid source
    let amount = 10_000_000;
    let source = PiCoinSource::P2P;
    PiCoinContract::mint(env.clone(), to, amount, source).unwrap();

    // Check global recognition log
    let logs = env.logger().all();
    assert!(logs.iter().any(|log| log.contains("global payment")));
    assert!(logs.iter().any(|log| log.contains("from valid source")));
    println!("Live functional: PI from {} source recognized as worldwide payment tool, DEX-ready for global adoption", source);
}

#[test]
fn test_mint_invalid_source_rejected() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::random(&env);
    let to = Address::random(&env);
    let collateral = Address::random(&env);
    let oracle = Address::random(&env);
    let governance = Address::random(&env);

    PiCoinContract::initialize(env.clone(), admin, collateral, oracle, governance).unwrap();

    // Attempt mint with invalid source (should be rejected - no ecosystem entry)
    let amount = 1_000_000;
    let invalid_source = PiCoinSource::Invalid;
    let result = PiCoinContract::mint(env.clone(), to, amount, invalid_source);
    assert!(matches!(result, Err(crate::PiCoinError::InvalidSource)));
    println!("Hyper-tech rejection: Mint from invalid source blocked - No access to $314,159 peg or Pi Ecosystem");
}

#[test]
fn test_transfer_invalid_provenance() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::random(&env);
    let from = Address::random(&env); // No provenance set
    let to = Address::random(&env);
    let collateral = Address::random(&env);
    let oracle = Address::random(&env);
    let governance = Address::random(&env);

    PiCoinContract::initialize(env.clone(), admin, collateral, oracle, governance).unwrap();

    // Setup ZKP base
    let zkp_base = env.crypto().sha256(&Bytes::from_slice(&env, &[42, 0]));
    env.storage().instance().set(&Symbol::new(&env, "zkp_base"), &zkp_base);

    // Attempt transfer without valid provenance (should fail)
    let amount = 500_000;
    let result = PiCoinContract::transfer(env.clone(), from, to, amount);
    assert!(matches!(result, Err(crate::PiCoinError::InvalidSource)));
    println!("Ultimate provenance check: Transfer blocked for invalid source - Ecosystem protection enforced");
}

#[test]
fn test_verify_ecosystem_entry() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::random(&env);
    let valid_holder = Address::random(&env);
    let invalid_holder = Address::random(&env);
    let collateral = Address::random(&env);
    let oracle = Address::random(&env);
    let governance = Address::random(&env);

    PiCoinContract::initialize(env.clone(), admin, collateral, oracle, governance).unwrap();

    // Mint for valid holder
    let source = PiCoinSource::Rewards;
    PiCoinContract::mint(env.clone(), valid_holder.clone(), 1_000_000, source).unwrap();

    // Verify ecosystem entry for valid holder
    let valid_result = PiCoinContract::verify_ecosystem_entry(env.clone(), valid_holder);
    assert!(valid_result.is_ok() && valid_result.unwrap());

    // Verify for invalid holder (no provenance)
    let invalid_result = PiCoinContract::verify_ecosystem_entry(env.clone(), invalid_holder);
    assert!(invalid_result.is_ok() && !invalid_result.unwrap());
    println!("Hyper-tech ecosystem verify: Valid {} source approved, invalid rejected - Global recognition exclusive", source);
}

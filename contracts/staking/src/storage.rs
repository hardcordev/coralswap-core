use soroban_sdk::{contracttype, Address, Env};

use coralswap_shared::{INSTANCE_TTL_EXTEND_TO, INSTANCE_TTL_THRESHOLD};

// ---------------------------------------------------------------------------
// Storage key enum
// ---------------------------------------------------------------------------

/// All persistent keys used by the staking contract.
///
/// `DataKey` is serialised directly into Soroban instance storage.  Every
/// variant is documented with the type of value it maps to.
#[contracttype]
#[derive(Clone, Debug)]
pub enum DataKey {
    /// [`StakingConfig`] — written once on `initialize`, updated by admin.
    Config,
    /// `i128` — total LP tokens currently locked in the contract.
    TotalStaked,
    /// `i128` — per-user staked LP token balance, keyed by staker address.
    UserStake(Address),
    /// `i128` — per-user accrued but unclaimed reward balance, keyed by staker address.
    UserPendingRewards(Address),
    /// u32 — ledger number at which TTL was last extended, for TTL querying.
    LastTTLExtendLedger,
}

// ---------------------------------------------------------------------------
// Config struct
// ---------------------------------------------------------------------------

/// Persistent configuration written by `initialize` and updated by the admin.
///
/// Stored under [`DataKey::Config`] in instance storage.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct StakingConfig {
    /// Address of the LP token that users lock in this contract.
    pub lp_token: Address,
    /// Address of the token distributed as staking rewards.
    pub reward_token: Address,
    /// Reward emission rate in smallest token units per ledger.
    ///
    /// Governance may update this value via `set_reward_config`.  It is
    /// intentionally stored as a raw rate here — the contract does NOT
    /// implement reward accumulation in this iteration (see issue scope).
    pub reward_rate: u64,
    /// Address with admin privileges (can update reward config).
    pub admin: Address,
}

// ---------------------------------------------------------------------------
// Accessors
// ---------------------------------------------------------------------------

pub fn get_config(env: &Env) -> Option<StakingConfig> {
    env.storage().instance().get(&DataKey::Config)
}

pub fn set_config(env: &Env, config: &StakingConfig) {
    env.storage().instance().set(&DataKey::Config, config);
}

pub fn has_config(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Config)
}

/// Returns the total LP tokens staked across all users.  Defaults to `0`
/// before the first stake.
pub fn get_total_staked(env: &Env) -> i128 {
    env.storage().instance().get(&DataKey::TotalStaked).unwrap_or(0i128)
}

pub fn set_total_staked(env: &Env, amount: i128) {
    env.storage().instance().set(&DataKey::TotalStaked, &amount);
}

/// Returns the staked balance for `user`.  Defaults to `0` for addresses that
/// have never staked.
pub fn get_user_stake(env: &Env, user: &Address) -> i128 {
    env.storage().instance().get(&DataKey::UserStake(user.clone())).unwrap_or(0i128)
}

pub fn set_user_stake(env: &Env, user: &Address, amount: i128) {
    env.storage().instance().set(&DataKey::UserStake(user.clone()), &amount);
}

pub fn set_user_stake(env: &Env, user: &Address, amount: i128) {
    env.storage().instance().set(&DataKey::UserStake(user.clone()), &amount);
}

pub fn get_user_pending_rewards(env: &Env, user: &Address) -> i128 {
    env.storage().instance().get(&DataKey::UserPendingRewards(user.clone())).unwrap_or(0i128)
}

pub fn set_user_pending_rewards(env: &Env, user: &Address, amount: i128) {
    env.storage().instance().set(&DataKey::UserPendingRewards(user.clone()), &amount);
}

pub fn add_user_pending_rewards(env: &Env, user: &Address, amount: i128) {
    let current = storage::get_user_pending_rewards(&env, user);
    storage::set_user_pending_rewards(&env, user, current.checked_add(amount).expect("overflow"));
}

pub fn reset_user_pending_rewards(env: &Env, user: &Address) {
    storage::set_user_pending_rewards(&env, user, 0);
}

pub fn get_last_ttl_extend_ledger(env: &Env) -> u32 {
    env.storage().instance().get(&DataKey::LastTTLExtendLedger).unwrap_or(0)
}

pub fn set_last_ttl_extend_ledger(env: &Env, ledger: u32) {
    env.storage().instance().set(&DataKey::LastTTLExtendLedger, &ledger);
}

/// Extends the instance TTL using the shared policy (3.5 days threshold →
/// 7 days bump).  Must be called at the end of every mutating entry point so
/// the contract stays alive across idle periods.
pub fn extend_instance_ttl(env: &Env, current_ledger: u32) {
    env.storage().instance().extend_ttl(INSTANCE_TTL_THRESHOLD, INSTANCE_TTL_EXTEND_TO);
    storage::set_last_ttl_extend_ledger(env, current_ledger);
}

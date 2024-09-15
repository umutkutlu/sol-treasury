use anchor_lang::prelude::*;

#[constant]
pub const SEED: &str = "anchor";

#[constant]
pub const ACCESS_ROLES_SEED: &[u8] = b"access_roles_account";

#[constant]
pub const FEE_CONFIG_SEED: &[u8] = b"fee_config_account";

#[constant]
pub const TREASURY_SEED: &[u8] = b"treasury_account";

#[constant]
pub const TREASURY_HOLDER_SEED: &[u8] = b"treasury_holder_account";

#[constant]
pub const ATTACK_COUNTER_SEED: &[u8] = b"attack_counter_account";

#[constant]
pub const PENDING_ATTACKS_SEED: &[u8] = b"pending_attacks_account";

#[constant]
pub const ATTACK_DATA_SEED: &[u8] = b"attack_data_account";

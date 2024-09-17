use anchor_lang::prelude::*;
use trident_client::fuzzing::{anchor_lang, FuzzingError};
pub struct InitializeAccessRolesSnapshot<'info> {
    pub access_roles_account:
        Option<Account<'info, sol_treasury::state::access_roles::AccessRoles>>,
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
pub struct InitializeAttackCounterSnapshot<'info> {
    pub attack_counter_account:
        Option<Account<'info, sol_treasury::state::attack_counter::AttackCounter>>,
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
pub struct InitializeFeeConfigSnapshot<'info> {
    pub fee_config_account: Option<Account<'info, sol_treasury::state::fee_config::FeeConfig>>,
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
pub struct InitializePendingAttacksSnapshot<'info> {
    pub pending_attacks_account:
        Option<Account<'info, sol_treasury::state::pending_attacks::PendingAttacks>>,
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
pub struct InitializeTreasurySnapshot<'info> {
    pub treasury_account: Option<Account<'info, sol_treasury::state::treasury::Treasury>>,
    pub treasury_holder_account: Option<&'info AccountInfo<'info>>,
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
pub struct FinalizeAttackSnapshot<'info> {
    pub attack_data_account: Option<Account<'info, sol_treasury::state::attack_data::AttackData>>,
    pub pending_attacks_account:
        Account<'info, sol_treasury::state::pending_attacks::PendingAttacks>,
    pub attack_counter_account: Account<'info, sol_treasury::state::attack_counter::AttackCounter>,
    pub access_roles_account: Account<'info, sol_treasury::state::access_roles::AccessRoles>,
    pub treasury_holder_account: &'info AccountInfo<'info>,
    pub treasury_account: Account<'info, sol_treasury::state::treasury::Treasury>,
    pub payer: Signer<'info>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
pub struct RequestAttackSnapshot<'info> {
    pub pending_attacks_account:
        Account<'info, sol_treasury::state::pending_attacks::PendingAttacks>,
    pub treasury_holder_account: &'info AccountInfo<'info>,
    pub fee_config_account: Account<'info, sol_treasury::state::fee_config::FeeConfig>,
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}
pub struct SetAccessRolesSnapshot<'info> {
    pub access_roles_account: Account<'info, sol_treasury::state::access_roles::AccessRoles>,
    pub authority: Signer<'info>,
}
pub struct SetFeeConfigSnapshot<'info> {
    pub fee_config_account: Account<'info, sol_treasury::state::fee_config::FeeConfig>,
    pub access_roles_account: Account<'info, sol_treasury::state::access_roles::AccessRoles>,
    pub authority: Signer<'info>,
}
impl<'info> InitializeAccessRolesSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let access_roles_account: Option<
            anchor_lang::accounts::account::Account<sol_treasury::state::access_roles::AccessRoles>,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "access_roles_account".to_string(),
            ))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::account::Account::try_from(acc).map_err(|_| {
                        FuzzingError::CannotDeserializeAccount("access_roles_account".to_string())
                    })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "access_roles_account".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let payer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("payer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("payer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("payer".to_string()))?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        Ok(Self {
            access_roles_account,
            payer,
            system_program,
        })
    }
}
impl<'info> InitializeAttackCounterSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let attack_counter_account: Option<
            anchor_lang::accounts::account::Account<
                sol_treasury::state::attack_counter::AttackCounter,
            >,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "attack_counter_account".to_string(),
            ))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::account::Account::try_from(acc).map_err(|_| {
                        FuzzingError::CannotDeserializeAccount("attack_counter_account".to_string())
                    })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "attack_counter_account".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let payer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("payer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("payer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("payer".to_string()))?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        Ok(Self {
            attack_counter_account,
            payer,
            system_program,
        })
    }
}
impl<'info> InitializeFeeConfigSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let fee_config_account: Option<
            anchor_lang::accounts::account::Account<sol_treasury::state::fee_config::FeeConfig>,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "fee_config_account".to_string(),
            ))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::account::Account::try_from(acc).map_err(|_| {
                        FuzzingError::CannotDeserializeAccount("fee_config_account".to_string())
                    })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "fee_config_account".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let payer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("payer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("payer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("payer".to_string()))?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        Ok(Self {
            fee_config_account,
            payer,
            system_program,
        })
    }
}
impl<'info> InitializePendingAttacksSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let pending_attacks_account: Option<
            anchor_lang::accounts::account::Account<
                sol_treasury::state::pending_attacks::PendingAttacks,
            >,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "pending_attacks_account".to_string(),
            ))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::account::Account::try_from(acc).map_err(|_| {
                        FuzzingError::CannotDeserializeAccount(
                            "pending_attacks_account".to_string(),
                        )
                    })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "pending_attacks_account".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let payer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("payer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("payer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("payer".to_string()))?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        Ok(Self {
            pending_attacks_account,
            payer,
            system_program,
        })
    }
}
impl<'info> InitializeTreasurySnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let treasury_account: Option<
            anchor_lang::accounts::account::Account<sol_treasury::state::treasury::Treasury>,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "treasury_account".to_string(),
            ))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::account::Account::try_from(acc).map_err(|_| {
                        FuzzingError::CannotDeserializeAccount("treasury_account".to_string())
                    })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "treasury_account".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let treasury_holder_account = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "treasury_holder_account".to_string(),
            ))?
            .as_ref();
        let payer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("payer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("payer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("payer".to_string()))?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        Ok(Self {
            treasury_account,
            treasury_holder_account,
            payer,
            system_program,
        })
    }
}
impl<'info> FinalizeAttackSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let attack_data_account: Option<
            anchor_lang::accounts::account::Account<sol_treasury::state::attack_data::AttackData>,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "attack_data_account".to_string(),
            ))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::account::Account::try_from(acc).map_err(|_| {
                        FuzzingError::CannotDeserializeAccount("attack_data_account".to_string())
                    })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "attack_data_account".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let pending_attacks_account: anchor_lang::accounts::account::Account<
            sol_treasury::state::pending_attacks::PendingAttacks,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "pending_attacks_account".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound(
                "pending_attacks_account".to_string(),
            ))?
            .map_err(|_| {
                FuzzingError::CannotDeserializeAccount("pending_attacks_account".to_string())
            })?;
        let attack_counter_account: anchor_lang::accounts::account::Account<
            sol_treasury::state::attack_counter::AttackCounter,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "attack_counter_account".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound(
                "attack_counter_account".to_string(),
            ))?
            .map_err(|_| {
                FuzzingError::CannotDeserializeAccount("attack_counter_account".to_string())
            })?;
        let access_roles_account: anchor_lang::accounts::account::Account<
            sol_treasury::state::access_roles::AccessRoles,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "access_roles_account".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound(
                "access_roles_account".to_string(),
            ))?
            .map_err(|_| {
                FuzzingError::CannotDeserializeAccount("access_roles_account".to_string())
            })?;
        let treasury_holder_account = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "treasury_holder_account".to_string(),
            ))?
            .as_ref()
            .ok_or(FuzzingError::AccountNotFound(
                "treasury_holder_account".to_string(),
            ))?;
        let treasury_account: anchor_lang::accounts::account::Account<
            sol_treasury::state::treasury::Treasury,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "treasury_account".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound(
                "treasury_account".to_string(),
            ))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("treasury_account".to_string()))?;
        let payer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("payer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("payer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("payer".to_string()))?;
        let authority: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("authority".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("authority".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("authority".to_string()))?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        Ok(Self {
            attack_data_account,
            pending_attacks_account,
            attack_counter_account,
            access_roles_account,
            treasury_holder_account,
            treasury_account,
            payer,
            authority,
            system_program,
        })
    }
}
impl<'info> RequestAttackSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let pending_attacks_account: anchor_lang::accounts::account::Account<
            sol_treasury::state::pending_attacks::PendingAttacks,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "pending_attacks_account".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound(
                "pending_attacks_account".to_string(),
            ))?
            .map_err(|_| {
                FuzzingError::CannotDeserializeAccount("pending_attacks_account".to_string())
            })?;
        let treasury_holder_account = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "treasury_holder_account".to_string(),
            ))?
            .as_ref()
            .ok_or(FuzzingError::AccountNotFound(
                "treasury_holder_account".to_string(),
            ))?;
        let fee_config_account: anchor_lang::accounts::account::Account<
            sol_treasury::state::fee_config::FeeConfig,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "fee_config_account".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound(
                "fee_config_account".to_string(),
            ))?
            .map_err(|_| {
                FuzzingError::CannotDeserializeAccount("fee_config_account".to_string())
            })?;
        let player: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("player".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("player".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("player".to_string()))?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        Ok(Self {
            pending_attacks_account,
            treasury_holder_account,
            fee_config_account,
            player,
            system_program,
        })
    }
}
impl<'info> SetAccessRolesSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let access_roles_account: anchor_lang::accounts::account::Account<
            sol_treasury::state::access_roles::AccessRoles,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "access_roles_account".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound(
                "access_roles_account".to_string(),
            ))?
            .map_err(|_| {
                FuzzingError::CannotDeserializeAccount("access_roles_account".to_string())
            })?;
        let authority: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("authority".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("authority".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("authority".to_string()))?;
        Ok(Self {
            access_roles_account,
            authority,
        })
    }
}
impl<'info> SetFeeConfigSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let fee_config_account: anchor_lang::accounts::account::Account<
            sol_treasury::state::fee_config::FeeConfig,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "fee_config_account".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound(
                "fee_config_account".to_string(),
            ))?
            .map_err(|_| {
                FuzzingError::CannotDeserializeAccount("fee_config_account".to_string())
            })?;
        let access_roles_account: anchor_lang::accounts::account::Account<
            sol_treasury::state::access_roles::AccessRoles,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "access_roles_account".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound(
                "access_roles_account".to_string(),
            ))?
            .map_err(|_| {
                FuzzingError::CannotDeserializeAccount("access_roles_account".to_string())
            })?;
        let authority: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("authority".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("authority".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("authority".to_string()))?;
        Ok(Self {
            fee_config_account,
            access_roles_account,
            authority,
        })
    }
}

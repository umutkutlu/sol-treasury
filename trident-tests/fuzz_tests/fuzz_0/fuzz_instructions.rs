pub mod sol_treasury_fuzz_instructions {
    use crate::accounts_snapshots::*;
    use solana_sdk::native_token::LAMPORTS_PER_SOL;
    use trident_client::fuzzing::*;
    #[derive(Arbitrary, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
    pub enum FuzzInstruction {
        InitializeAccessRoles(InitializeAccessRoles),
        InitializeAttackCounter(InitializeAttackCounter),
        InitializeFeeConfig(InitializeFeeConfig),
        InitializePendingAttacks(InitializePendingAttacks),
        InitializeTreasury(InitializeTreasury),
        FinalizeAttack(FinalizeAttack),
        RequestAttack(RequestAttack),
        SetAccessRoles(SetAccessRoles),
        SetFeeConfig(SetFeeConfig),
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeAccessRoles {
        pub accounts: InitializeAccessRolesAccounts,
        pub data: InitializeAccessRolesData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeAccessRolesAccounts {
        pub access_roles_account: AccountId,
        pub payer: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeAccessRolesData {
        pub initial_owner: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeAttackCounter {
        pub accounts: InitializeAttackCounterAccounts,
        pub data: InitializeAttackCounterData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeAttackCounterAccounts {
        pub attack_counter_account: AccountId,
        pub payer: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeAttackCounterData {}
    #[derive(Arbitrary, Debug)]
    pub struct InitializeFeeConfig {
        pub accounts: InitializeFeeConfigAccounts,
        pub data: InitializeFeeConfigData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeFeeConfigAccounts {
        pub fee_config_account: AccountId,
        pub payer: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeFeeConfigData {
        pub initial_fee_holder: AccountId,
        pub initial_fee_per_thousand: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializePendingAttacks {
        pub accounts: InitializePendingAttacksAccounts,
        pub data: InitializePendingAttacksData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializePendingAttacksAccounts {
        pub pending_attacks_account: AccountId,
        pub payer: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializePendingAttacksData {}
    #[derive(Arbitrary, Debug)]
    pub struct InitializeTreasury {
        pub accounts: InitializeTreasuryAccounts,
        pub data: InitializeTreasuryData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeTreasuryAccounts {
        pub treasury_account: AccountId,
        pub treasury_holder_account: AccountId,
        pub payer: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeTreasuryData {}
    #[derive(Arbitrary, Debug)]
    pub struct FinalizeAttack {
        pub accounts: FinalizeAttackAccounts,
        pub data: FinalizeAttackData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct FinalizeAttackAccounts {
        pub attack_data_account: AccountId,
        pub pending_attacks_account: AccountId,
        pub attack_counter_account: AccountId,
        pub access_roles_account: AccountId,
        pub treasury_holder_account: AccountId,
        pub treasury_account: AccountId,
        pub payer: AccountId,
        pub authority: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct FinalizeAttackData {
        pub random_number: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct RequestAttack {
        pub accounts: RequestAttackAccounts,
        pub data: RequestAttackData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct RequestAttackAccounts {
        pub pending_attacks_account: AccountId,
        pub treasury_holder_account: AccountId,
        pub fee_config_account: AccountId,
        pub player: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct RequestAttackData {
        pub attack_amount: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct SetAccessRoles {
        pub accounts: SetAccessRolesAccounts,
        pub data: SetAccessRolesData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct SetAccessRolesAccounts {
        pub access_roles_account: AccountId,
        pub authority: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct SetAccessRolesData {
        pub new_owner: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct SetFeeConfig {
        pub accounts: SetFeeConfigAccounts,
        pub data: SetFeeConfigData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct SetFeeConfigAccounts {
        pub fee_config_account: AccountId,
        pub access_roles_account: AccountId,
        pub authority: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct SetFeeConfigData {
        pub new_fee_holder: AccountId,
        pub new_fee_per_thousand: u64,
    }
    impl<'info> IxOps<'info> for InitializeAccessRoles {
        type IxData = sol_treasury::instruction::InitializeAccessRoles;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = InitializeAccessRolesSnapshot<'info>;
        fn get_data(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let initial_owner = fuzz_accounts.initial_owner.get_or_create_account(
                self.data.initial_owner,
                client,
                10 * LAMPORTS_PER_SOL,
            );
            let data = sol_treasury::instruction::InitializeAccessRoles {
                initial_owner: initial_owner.pubkey(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let payer = fuzz_accounts.payer.get_or_create_account(
                self.accounts.payer,
                client,
                10 * LAMPORTS_PER_SOL,
            );

            let access_roles_account = fuzz_accounts
                .access_roles_account
                .get_or_create_account(
                    self.accounts.access_roles_account,
                    &[sol_treasury::constants::ACCESS_ROLES_SEED],
                    &sol_treasury::ID,
                )
                .unwrap();

            let signers = vec![payer.clone()];
            let acc_meta = sol_treasury::accounts::InitializeAccessRoles {
                access_roles_account: access_roles_account.pubkey(),
                payer: payer.pubkey(),
                system_program: solana_sdk::system_program::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for InitializeAttackCounter {
        type IxData = sol_treasury::instruction::InitializeAttackCounter;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = InitializeAttackCounterSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = sol_treasury::instruction::InitializeAttackCounter {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let payer = fuzz_accounts.payer.get_or_create_account(
                self.accounts.payer,
                client,
                10 * LAMPORTS_PER_SOL,
            );

            let attack_counter_account = fuzz_accounts
                .attack_counter_account
                .get_or_create_account(
                    self.accounts.attack_counter_account,
                    &[sol_treasury::constants::ATTACK_COUNTER_SEED],
                    &sol_treasury::ID,
                )
                .unwrap();

            let signers = vec![payer.clone()];
            let acc_meta = sol_treasury::accounts::InitializeAttackCounter {
                attack_counter_account: attack_counter_account.pubkey(),
                payer: payer.pubkey(),
                system_program: solana_sdk::system_program::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for InitializeFeeConfig {
        type IxData = sol_treasury::instruction::InitializeFeeConfig;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = InitializeFeeConfigSnapshot<'info>;
        fn get_data(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let initial_fee_holder = fuzz_accounts.initial_fee_holder.get_or_create_account(
                self.data.initial_fee_holder,
                client,
                10 * LAMPORTS_PER_SOL,
            );
            let data = sol_treasury::instruction::InitializeFeeConfig {
                initial_fee_holder: initial_fee_holder.pubkey(),
                initial_fee_per_thousand: self.data.initial_fee_per_thousand,
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let payer = fuzz_accounts.payer.get_or_create_account(
                self.accounts.payer,
                client,
                10 * LAMPORTS_PER_SOL,
            );

            let fee_config_account = fuzz_accounts
                .fee_config_account
                .get_or_create_account(
                    self.accounts.fee_config_account,
                    &[sol_treasury::constants::FEE_CONFIG_SEED],
                    &sol_treasury::ID,
                )
                .unwrap();

            let signers = vec![payer.clone()];
            let acc_meta = sol_treasury::accounts::InitializeFeeConfig {
                fee_config_account: fee_config_account.pubkey(),
                payer: payer.pubkey(),
                system_program: solana_sdk::system_program::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for InitializePendingAttacks {
        type IxData = sol_treasury::instruction::InitializePendingAttacks;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = InitializePendingAttacksSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = sol_treasury::instruction::InitializePendingAttacks {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let payer = fuzz_accounts.payer.get_or_create_account(
                self.accounts.payer,
                client,
                10 * LAMPORTS_PER_SOL,
            );

            let pending_attacks_account = fuzz_accounts
                .pending_attacks_account
                .get_or_create_account(
                    self.accounts.pending_attacks_account,
                    &[sol_treasury::constants::PENDING_ATTACKS_SEED],
                    &sol_treasury::ID,
                )
                .unwrap();

            let signers = vec![payer.clone()];
            let acc_meta = sol_treasury::accounts::InitializePendingAttacks {
                pending_attacks_account: pending_attacks_account.pubkey(),
                payer: payer.pubkey(),
                system_program: solana_sdk::system_program::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for InitializeTreasury {
        type IxData = sol_treasury::instruction::InitializeTreasury;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = InitializeTreasurySnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = sol_treasury::instruction::InitializeTreasury {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let payer = fuzz_accounts.payer.get_or_create_account(
                self.accounts.payer,
                client,
                10 * LAMPORTS_PER_SOL,
            );

            let treasury_account = fuzz_accounts
                .treasury_account
                .get_or_create_account(
                    self.accounts.treasury_account,
                    &[sol_treasury::constants::TREASURY_SEED],
                    &sol_treasury::ID,
                )
                .unwrap();

            let treasury_holder_account = fuzz_accounts
                .treasury_holder_account
                .get_or_create_account(
                    self.accounts.treasury_holder_account,
                    &[sol_treasury::constants::TREASURY_HOLDER_SEED],
                    &sol_treasury::ID,
                )
                .unwrap();

            let signers = vec![payer.clone()];
            let acc_meta = sol_treasury::accounts::InitializeTreasury {
                treasury_account: treasury_account.pubkey(),
                treasury_holder_account: treasury_holder_account.pubkey(),
                payer: payer.pubkey(),
                system_program: solana_sdk::system_program::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for FinalizeAttack {
        type IxData = sol_treasury::instruction::FinalizeAttack;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = FinalizeAttackSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = sol_treasury::instruction::FinalizeAttack {
                random_number: self.data.random_number,
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let payer = fuzz_accounts.payer.get_or_create_account(
                self.accounts.payer,
                client,
                10 * LAMPORTS_PER_SOL,
            );

            let authority = fuzz_accounts.authority.get_or_create_account(
                self.accounts.authority,
                client,
                10 * LAMPORTS_PER_SOL,
            );

            let attack_data_account = fuzz_accounts
                .attack_data_account
                .get_or_create_account(
                    self.accounts.attack_data_account,
                    &[sol_treasury::constants::ATTACK_DATA_SEED],
                    &sol_treasury::ID,
                )
                .unwrap();

            let pending_attacks_account = fuzz_accounts
                .pending_attacks_account
                .get_or_create_account(
                    self.accounts.pending_attacks_account,
                    &[sol_treasury::constants::PENDING_ATTACKS_SEED],
                    &sol_treasury::ID,
                )
                .unwrap();

            let attack_counter_account = fuzz_accounts
                .attack_counter_account
                .get_or_create_account(
                    self.accounts.attack_counter_account,
                    &[sol_treasury::constants::ATTACK_COUNTER_SEED],
                    &sol_treasury::ID,
                )
                .unwrap();

            let access_roles_account = fuzz_accounts
                .access_roles_account
                .get_or_create_account(
                    self.accounts.access_roles_account,
                    &[sol_treasury::constants::ACCESS_ROLES_SEED],
                    &sol_treasury::ID,
                )
                .unwrap();

            let treasury_holder_account = fuzz_accounts
                .treasury_holder_account
                .get_or_create_account(
                    self.accounts.treasury_holder_account,
                    &[sol_treasury::constants::TREASURY_HOLDER_SEED],
                    &sol_treasury::ID,
                )
                .unwrap();

            let treasury_account = fuzz_accounts
                .treasury_account
                .get_or_create_account(
                    self.accounts.treasury_account,
                    &[sol_treasury::constants::TREASURY_SEED],
                    &sol_treasury::ID,
                )
                .unwrap();

            let signers = vec![payer.clone(), authority.clone()];
            let acc_meta = sol_treasury::accounts::FinalizeAttack {
                attack_data_account: attack_data_account.pubkey(),
                pending_attacks_account: pending_attacks_account.pubkey(),
                attack_counter_account: attack_counter_account.pubkey(),
                access_roles_account: access_roles_account.pubkey(),
                treasury_holder_account: treasury_holder_account.pubkey(),
                treasury_account: treasury_account.pubkey(),
                payer: payer.pubkey(),
                authority: authority.pubkey(),
                system_program: solana_sdk::system_program::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for RequestAttack {
        type IxData = sol_treasury::instruction::RequestAttack;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = RequestAttackSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = sol_treasury::instruction::RequestAttack {
                attack_amount: self.data.attack_amount,
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let player = fuzz_accounts.player.get_or_create_account(
                self.accounts.player,
                client,
                10 * LAMPORTS_PER_SOL,
            );

            let pending_attacks_account = fuzz_accounts
                .pending_attacks_account
                .get_or_create_account(
                    self.accounts.pending_attacks_account,
                    &[sol_treasury::constants::PENDING_ATTACKS_SEED],
                    &sol_treasury::ID,
                )
                .unwrap();

            let treasury_holder_account = fuzz_accounts
                .treasury_holder_account
                .get_or_create_account(
                    self.accounts.treasury_holder_account,
                    &[sol_treasury::constants::TREASURY_HOLDER_SEED],
                    &sol_treasury::ID,
                )
                .unwrap();

            let fee_config_account = fuzz_accounts
                .fee_config_account
                .get_or_create_account(
                    self.accounts.fee_config_account,
                    &[sol_treasury::constants::FEE_CONFIG_SEED],
                    &sol_treasury::ID,
                )
                .unwrap();

            let signers = vec![player.clone()];
            let acc_meta = sol_treasury::accounts::RequestAttack {
                pending_attacks_account: pending_attacks_account.pubkey(),
                treasury_holder_account: treasury_holder_account.pubkey(),
                fee_config_account: fee_config_account.pubkey(),
                player: player.pubkey(),
                system_program: solana_sdk::system_program::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for SetAccessRoles {
        type IxData = sol_treasury::instruction::SetAccessRoles;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = SetAccessRolesSnapshot<'info>;
        fn get_data(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let new_owner = fuzz_accounts.new_owner.get_or_create_account(
                self.data.new_owner,
                client,
                10 * LAMPORTS_PER_SOL,
            );
            let data = sol_treasury::instruction::SetAccessRoles {
                new_owner: new_owner.pubkey(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let authority = fuzz_accounts.authority.get_or_create_account(
                self.accounts.authority,
                client,
                10 * LAMPORTS_PER_SOL,
            );

            let access_roles_account = fuzz_accounts
                .access_roles_account
                .get_or_create_account(
                    self.accounts.access_roles_account,
                    &[sol_treasury::constants::ACCESS_ROLES_SEED],
                    &sol_treasury::ID,
                )
                .unwrap();

            let signers = vec![authority.clone()];
            let acc_meta = sol_treasury::accounts::SetAccessRoles {
                access_roles_account: access_roles_account.pubkey(),
                authority: authority.pubkey(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for SetFeeConfig {
        type IxData = sol_treasury::instruction::SetFeeConfig;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = SetFeeConfigSnapshot<'info>;
        fn get_data(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let new_fee_holder = fuzz_accounts.new_fee_holder.get_or_create_account(
                self.data.new_fee_holder,
                client,
                10 * LAMPORTS_PER_SOL,
            );
            let data = sol_treasury::instruction::SetFeeConfig {
                new_fee_holder: new_fee_holder.pubkey(),
                new_fee_per_thousand: self.data.new_fee_per_thousand,
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let authority = fuzz_accounts.authority.get_or_create_account(
                self.accounts.authority,
                client,
                10 * LAMPORTS_PER_SOL,
            );

            let fee_config_account = fuzz_accounts
                .fee_config_account
                .get_or_create_account(
                    self.accounts.fee_config_account,
                    &[sol_treasury::constants::FEE_CONFIG_SEED],
                    &sol_treasury::ID,
                )
                .unwrap();

            let access_roles_account = fuzz_accounts
                .access_roles_account
                .get_or_create_account(
                    self.accounts.access_roles_account,
                    &[sol_treasury::constants::ACCESS_ROLES_SEED],
                    &sol_treasury::ID,
                )
                .unwrap();

            let signers = vec![authority.clone()];
            let acc_meta = sol_treasury::accounts::SetFeeConfig {
                fee_config_account: fee_config_account.pubkey(),
                access_roles_account: access_roles_account.pubkey(),
                authority: authority.pubkey(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    #[doc = r" Use AccountsStorage<T> where T can be one of:"]
    #[doc = r" Keypair, PdaStore, TokenStore, MintStore, ProgramStore"]
    #[derive(Default)]
    pub struct FuzzAccounts {
        access_roles_account: AccountsStorage<PdaStore>,
        attack_counter_account: AccountsStorage<PdaStore>,
        attack_data_account: AccountsStorage<PdaStore>,
        authority: AccountsStorage<Keypair>,
        fee_config_account: AccountsStorage<PdaStore>,
        payer: AccountsStorage<Keypair>,
        pending_attacks_account: AccountsStorage<PdaStore>,
        player: AccountsStorage<Keypair>,
        //system_program: AccountsStorage<todo!()>,
        treasury_account: AccountsStorage<PdaStore>,
        treasury_holder_account: AccountsStorage<PdaStore>,
        // Additional parameters
        initial_owner: AccountsStorage<Keypair>,
        initial_fee_holder: AccountsStorage<Keypair>,
        new_owner: AccountsStorage<Keypair>,
        new_fee_holder: AccountsStorage<Keypair>,
    }
}

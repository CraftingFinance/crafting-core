#![cfg_attr(not(feature = "std"), no_std)]
// pub use self::param::Param;

use ink_lang as ink;

#[ink::contract]
pub mod param {
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_storage::collections::HashMap as StorageHashMap;

    /// Money matters.
    pub const UNITS: Balance = 10_000_000_000;
    pub const DOLLARS: Balance = UNITS;            // 10_000_000_000
    pub const CENTS: Balance = DOLLARS / 100;      // 100_000_000
    pub const MILLICENTS: Balance = CENTS / 1_000; // 100_000

    /// Time and blocks.
    pub const MILLISECS_PER_BLOCK: Timestamp = 6000;

    // These time units are defined in number of blocks.
    pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
    pub const HOURS: BlockNumber = MINUTES * 60;
    pub const DAYS: BlockNumber = HOURS * 24;

    /// Fee-related.
    pub const FEE_PRECISION: Balance = 100_000_000;

    #[ink(storage)]
    pub struct Param {
        collateral_assets: StorageHashMap<AccountId, u32>,
        synthetic_assets: StorageHashMap<AccountId, u8>,
        leverage_ratio: (u8, u8),
        interest_fee: u8,
        transaction_fee: u8,
        owner: AccountId,
    }

    impl Param {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                collateral_assets: StorageHashMap::new(),
                synthetic_assets: StorageHashMap::new(),
                leverage_ratio: (1, 10),
                interest_fee: 0,
                transaction_fee: 3,
                owner: Self::env().caller(),
            }
        }

        #[ink(message)]
        pub fn get_collateral_asset(&self, asset: AccountId) -> u32 {
            assert_ne!(asset, Default::default());

            self.collateral_assets.get(&asset).copied().unwrap_or(0)
        }

        #[ink(message)]
        pub fn set_collateral_asset(&mut self, asset: AccountId, ratio: u32) {
            self.is_owner();
            assert_ne!(asset, Default::default());

            self.collateral_assets.insert(asset, ratio);
        }

        #[ink(message)]
        pub fn remove_collateral_asset(&mut self, asset: AccountId) {
            self.is_owner();
            assert_ne!(asset, Default::default());

            self.collateral_assets.take(&asset);
        }

        #[ink(message)]
        pub fn is_effective_collateral_asset(&self, asset: AccountId) -> bool {
            assert_ne!(asset, Default::default());

            if self.collateral_assets.contains_key(&asset) {
                return true;
            }

            false
        }

        #[ink(message)]
        pub fn get_synthetic_asset(&self, asset: AccountId) -> u8 {
            assert_ne!(asset, Default::default());

            self.synthetic_assets.get(&asset).copied().unwrap_or(0)
        }

        #[ink(message)]
        pub fn set_synthetic_asset(&mut self, asset: AccountId, status: u8) {
            self.is_owner();
            assert_ne!(asset, Default::default());

            self.synthetic_assets.insert(asset, status);
        }

        #[ink(message)]
        pub fn remove_synthetic_asset(&mut self, asset: AccountId) {
            self.is_owner();
            assert_ne!(asset, Default::default());

            self.synthetic_assets.take(&asset);
        }

        #[ink(message)]
        pub fn is_effective_synthetic_asset(&self, asset: AccountId) -> bool {
            assert_ne!(asset, Default::default());

            if self.synthetic_assets.contains_key(&asset) {
                return true;
            }

            false
        }

        #[ink(message)]
        pub fn get_leverage_ratio(&self) -> (u8, u8) {
            self.leverage_ratio
        }

        #[ink(message)]
        pub fn set_leverage_ratio(&mut self, min: u8, max: u8) {
            self.is_owner();

            self.leverage_ratio = (min, max);
        }

        #[ink(message)]
        pub fn get_interest_fee(&self) -> u8 {
            self.interest_fee
        }

        #[ink(message)]
        pub fn set_interest_fee(&mut self, interest_fee: u8) {
            self.is_owner();

            self.interest_fee = interest_fee;
        }

        #[ink(message)]
        pub fn get_transaction_fee(&self) -> u8 {
            self.transaction_fee
        }

        #[ink(message)]
        pub fn set_transaction_fee(&mut self, transaction_fee: u8) {
            self.is_owner();

            self.transaction_fee = transaction_fee;
        }

        #[ink(message)]
        pub fn transfer_ownership(&mut self, new_owner: AccountId) {
            self.is_owner();
            assert_ne!(new_owner, Default::default());

            self.owner = new_owner;
        }

        fn is_owner(&self) {
            assert_eq!(self.owner, self.env().caller());
        }
    }
}
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap};
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, StorageUsage};

pub mod ft_core;
pub mod events;
pub mod metadata;
pub mod storage;
pub mod internal;

use crate::metadata::*;
use crate::events::*;

/// The image URL for the default icon
const DATA_IMAGE_SVG_GT_ICON: &str = "data:image/png;base64,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";

/// The specific version of the standard we're using
pub const FT_METADATA_SPEC: &str = "ft-1.0.0";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    /// Keep track of each account's balances
    pub accounts: LookupMap<AccountId, Balance>,

    /// Total supply of all tokens.
    pub total_supply: Balance,

    /// The bytes for the largest possible account ID that can be registered on the contract 
    pub bytes_for_longest_account_id: StorageUsage,

    /// Metadata for the contract itself
    pub metadata: LazyOption<FungibleTokenMetadata>,
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    Accounts,
    Metadata
}

#[near_bindgen]
impl Contract {
    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// default metadata (for example purposes only).
    #[init]
    pub fn new_default_meta(owner_id: AccountId, total_supply: U128) -> Self {
        // Calls the other function "new: with some default metadata and the owner_id & total supply passed in 
        Self::new(
            owner_id,
            total_supply,
            FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: "Anil6_NEAR_Token".to_string(),
                symbol: "AP6".to_string(),
                icon: Some(DATA_IMAGE_SVG_GT_ICON.to_string()),
                reference: None,
                reference_hash: None,
                decimals: 24,
            },
        )
    }

    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// the given fungible token metadata.
    #[init]
    pub fn new(
        owner_id: AccountId,
        total_supply: U128,
        metadata: FungibleTokenMetadata,
    ) -> Self {
        // Create a variable of type Self with all the fields initialized. 
        let mut this = Self {
            // Set the total supply
            total_supply: total_supply.0,
            // Set the bytes for the longest account ID to 0 temporarily until it's calculated later
            bytes_for_longest_account_id: 0,
            // Storage keys are simply the prefixes used for the collections. This helps avoid data collision
            accounts: LookupMap::new(StorageKey::Accounts.try_to_vec().unwrap()),
            metadata: LazyOption::new(
                StorageKey::Metadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
        };

        // Measure the bytes for the longest account ID and store it in the contract.
        this.measure_bytes_for_longest_account_id();

        // Register the owner's account and set their balance to the total supply.
        this.internal_register_account(&owner_id);
        this.internal_deposit(&owner_id, total_supply.into());
        
        // Emit an event showing that the FTs were minted
        FtMint {
            owner_id: &owner_id,
            amount: &total_supply,
            memo: Some("Initial token supply is minted"),
        }
        .emit();

        // Return the Contract object
        this
    }
}
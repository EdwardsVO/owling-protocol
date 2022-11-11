// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, PanicOnDefault};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, AccountId, Balance, Promise
};
use near_sdk::serde_json::json;

mod metadata;
mod enumeration;
mod form;
mod internal;

use crate::metadata::*;
use crate::internal::*;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    pub form_by_id: UnorderedMap<U128, Form>,
    pub forms_by_creator: LookupMap<AccountId, UnorderedSet<Form>>,
    pub metadata:LazyOption<OwlingContractMetadata>,
}

#[derive(BorshSerialize)]
pub enum StorageKey {
    FormsById,
    FormsByCreator,
    OwlingMetadata
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    #[init]
    pub fn new_meta(owner_id: AccountId) -> Self{
        Self::new(
            owner_id,
            OwlingContractMetadata {
                name: "Owling".to_string()
            },
        )
    }

    #[init]
    pub fn new(owner_id: AccountId, metadata: OwlingContractMetadata) -> Self{
        let this: Contract = Self {
            owner_id: owner_id,
            form_by_id: UnorderedMap::new(
                StorageKey::FormsById.try_to_vec().unwrap(),
            ),
            forms_by_creator: LookupMap::new(StorageKey::FormsByCreator.try_to_vec().unwrap()),
            metadata: LazyOption::new(
                StorageKey::OwlingMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
        };
        this
    }
}
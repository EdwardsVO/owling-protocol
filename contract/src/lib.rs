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

//Developement modules
//Contains all the information about structs 
mod metadata;

//Contains get and view methods
mod enumeration;

//Contains all the contract logic 
mod form;

//Functions called by modules that might not necessarily be inside the contract implementation
//Helps the contract readability
mod internal;

//Functions and structs used on the project 
use crate::metadata::*;
use crate::internal::*;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //Contract owner id
    pub owner_id: AccountId,

    //Storage that save the {Form} struct with the id as key
    //Dev {Form} in Metadata module
    pub form_by_id: UnorderedMap<U128, Form>,

    //Save an UnorderedSet of forms per user id or creator
    pub forms_by_creator: LookupMap<AccountId, UnorderedSet<Form>>,

    //Save the answers by id, each key is the answer id 
    pub answer_by_id: UnorderedMap<U128, Answer>,

    //Save the users answer, each key is the user account id
    pub answers_by_user: LookupMap<AccountId, UnorderedSet<Answer>>,

    //Contract metadata
    pub metadata:LazyOption<OwlingContractMetadata>,
}

#[derive(BorshSerialize)]
pub enum StorageKey {
    FormsById,
    FormsByCreator,
    OwlingMetadata,
    AnswerById,
    AnswerByUser
}

// Initialize the contract structure
#[near_bindgen]
impl Contract {

    // Automatically intialize storages calling the method new()
    #[init]
    pub fn new_meta(owner_id: AccountId) -> Self{
        Self::new(
            owner_id,
            OwlingContractMetadata {
                name: "Owling".to_string()
            },
        )
    }

    //Initializes the storage through the StorageKey enum
    #[init]
    pub fn new(owner_id: AccountId, metadata: OwlingContractMetadata) -> Self{
        let this: Contract = Self {
            owner_id: owner_id,
            form_by_id: UnorderedMap::new(
                StorageKey::FormsById.try_to_vec().unwrap(),
            ),
            forms_by_creator: LookupMap::new(
                StorageKey::FormsByCreator.try_to_vec().unwrap()
            ),
            metadata: LazyOption::new(
                StorageKey::OwlingMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
            answer_by_id: UnorderedMap::new(
                StorageKey::AnswerById.try_to_vec().unwrap(),
            ),
            answers_by_user: LookupMap::new(
                StorageKey::AnswerByUser.try_to_vec().unwrap()
            )
        };
        this
    }
}
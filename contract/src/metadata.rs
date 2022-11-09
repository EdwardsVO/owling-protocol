use crate::*;


//Metadata need for init the contract
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct OwlingContractMetadata { 
    pub name: String, //Only for initialization purposes
}

//Metadata need for init the contract
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Form { 
    pub title: String //Only for initialization purposes
}
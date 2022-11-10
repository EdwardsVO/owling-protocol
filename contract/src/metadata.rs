use crate::*;

pub type FormId = U128;
pub type AnswerId = u128;
pub type Date = u64;
pub type AnswerPoints = u128;

//Metadata need for init the contract
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct OwlingContractMetadata { 
    pub name: String, //Only for initialization purposes
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Form { 
    pub id: FormId,
    //To identigy the form
    pub title: String, 
    //For filtering information
    pub creation: Date,  
    //Questions will be created by creator and it will  be stored in an  Array
    //every position [n] will contain the question id
    pub questions: Vec<String>,
    //Possible answers for every question. They will be stored as matrix MxN.
    //Every vector index must match with the corresponded question 
    //One question could have several answers
    //Answers and Questions must have the same lenght
    pub possibly_answers: Vec<Vec<String>>,
    //Points that represents each answer
    pub answers_points: Vec<Vec<AnswerPoints>>,
    //There are multiple results, the answer points will be sum and divided by the results lenght
    pub results: Vec<String>,
    //Results URLs
    //Each image index must match with the corresponded result
    pub results_images: Vec<String>
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FormAnswer {
    pub id: AnswerId,
    //Link with the form 
    pub form_id: FormId,
    //User link
    pub wallet_id: AccountId,
    //Accoumulate points
    pub total_points: U128,
    //Final result title 
    pub final_result: String,
    //Image result
    pub final_image: String

}
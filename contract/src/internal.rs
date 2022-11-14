use crate::*;


// Helps concat arrays of strings
pub(crate) fn concat_array_strings(first: &[Vec<String>], second: &[Vec<String>]) -> Vec<Vec<String>> {
    [first, second].concat()
}

// Help concat strings
pub(crate) fn concat_strings(first: &[String], second: &[String]) -> Vec<String> {
    [first, second].concat()
}

// Help concat arrays of numbers u128 type
pub(crate) fn concat_array_u128(first: &[Vec<u128>], second: &[Vec<u128>]) -> Vec<Vec<u128>> {
    [first, second].concat()
}

// refund the initial deposit based on the amount of storage that was used up
pub(crate) fn refund_deposit(storage_used: u64) {
    // get how much it would cost to store the information
    let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
    // get the attached deposit
    let attached_deposit = env::attached_deposit();

    // make sure that the attached deposit is greater than or equal to the required cost
    assert!(
        required_cost <= attached_deposit,
        "Must attach {} yoctoNEAR to cover storage",
        required_cost,
    );

    // get the refund amount from the attached deposit - required cost
    let refund = attached_deposit - required_cost;

    // if the refund is greater than 1 yocto NEAR, we refund the predecessor that amount
    if refund > 1 {
        Promise::new(env::predecessor_account_id()).transfer(refund);
    }
}

#[near_bindgen]
impl Contract {

    // Save form in {form_by_id} and {forms_by_creator}
    pub(crate) fn save_form(&mut self, form: Form) {

        // Get the account id
        let creator = env::signer_account_id();

        // Insert the form created with its id
        self.form_by_id.insert(&form.id, &form);

        // Insert the form in the struct that contains all the creators form
        // If creator hasn't initialized this struct it will be created 
        // the the form it's inserted
        let mut forms_set = self.forms_by_creator.get(&creator).unwrap_or_else(|| {

            // Initialize the struct
            UnorderedSet::new(
                StorageKey::FormsByCreator.try_to_vec().unwrap()
            )
        });
        forms_set.insert(&form);

        // Insert the form and the struct im storage
        self.forms_by_creator.insert(&creator, &forms_set);
    }

    pub(crate) fn save_answer(&mut self, answer: Answer) {
        // Save answer in {answer_by_id} and {answer_by_user}
        let user = env::signer_account_id();

        // Insert the answer created with its id
        self.answer_by_id.insert(&U128(answer.id), &answer);

        // Insert the answer in the struct that contains all the user answers
        // If user hasn't initialized this struct it will be created 
        // the the answer it's inserted
        let mut answer_set = self.answers_by_user.get(&user).unwrap_or_else(|| {

            // Initialize the struct
            UnorderedSet::new(
                StorageKey::AnswerByUser.try_to_vec().unwrap()
            )
        });
        answer_set.insert(&answer);
        
        // Insert the answer and the struct in storage
        self.answers_by_user.insert(&user, &answer_set);
    }
}

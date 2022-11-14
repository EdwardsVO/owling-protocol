use crate::*;

#[near_bindgen]
impl Contract {
    // Get the total forms amount created
    pub fn forms_total_supply(&self) -> U128 {
        return U128(self.form_by_id.len() as u128);
    }

    // Get a form by a given ID 
    // Receives the id 
    pub fn form_by_id(&self, form_id: FormId) -> Option<Form> {

        // If exist return the searched form if not return null
        if let Some(form) = self.form_by_id.get(&form_id) {
            return Some(form);
        } else {
            None
        }
    }

    // Get the creator forms
    // Receives the creator account id 
    pub fn forms_by_creator(&self, creator_id: AccountId) -> Vec<Form> {
        
        let forms_by_creator = self.forms_by_creator.get(&creator_id);
        if let Some(forms_by_creator) = forms_by_creator {
            return forms_by_creator.to_vec();
        } else {
            return vec![];
        };
    }

    // Get answer through answer id
    pub fn answer_by_id(&self, answer_id: AnswerId) -> Option<Answer> {

        // If exist return the searched form if not return null
        if let Some(answer) = self.answer_by_id.get(&U128(answer_id)) {
            return Some(answer);
        } else {
            None
        }
    }

    pub fn answers_by_user(&self, user_id: AccountId) -> Vec<Answer> {
        
        let answers_by_user = self.answers_by_user.get(&user_id);
        if let Some(answers_by_user) = answers_by_user {
            return answers_by_user.to_vec();
        } else {
            return vec![];
        };
    }

}
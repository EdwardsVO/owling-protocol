use crate::*;

#[near_bindgen]
impl Contract {
    //Get the total forms amount created
    pub fn forms_total_supply(&self) -> U128 {
        return U128(self.form_by_id.len() as u128);
    }

    //Get a form by a given ID 
    //Receives the id 
    pub fn form_by_id(&self, form_id: FormId) -> Option<Form> {

        //If exist return the searched form if not return null
        if let Some(form) = self.form_by_id.get(&form_id) {
            return Some(form);
        } else {
            None
        }
    }

    //Get the creator forms
    //Receives the creator account id 
    pub fn forms_by_creator(&self, creator_id: AccountId) -> Vec<Form> {
        
        let forms_by_creator = self.forms_by_creator.get(&creator_id);
        if let Some(forms_by_creator) = forms_by_creator {
            return forms_by_creator.to_vec();
        } else {
            return vec![];
        };


    }

}
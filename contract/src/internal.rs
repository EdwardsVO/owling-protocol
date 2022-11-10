use crate::*;

#[near_bindgen]
impl Contract {
    pub(crate) fn save_form(&mut self, form: Form) {
        let creator = env::signer_account_id();
        self.forms_by_id.insert(&form.id, &form);
        let mut forms_set = self.forms_by_creator.get(&creator).unwrap_or_else(|| {
            UnorderedSet::new(
                StorageKey::FormsByCreator.try_to_vec().unwrap()
            )
        });
        forms_set.insert(&form);
        self.forms_by_creator.insert(&creator, &forms_set);
    }
}

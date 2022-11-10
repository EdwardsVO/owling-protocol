use crate::*;


#[near_bindgen]
impl Contract {


    pub fn create_form(
        &mut self,
        title: String,
        questions: Vec<String>,
        possibly_answers: Vec<Vec<String>>,
        answer_points: Vec<Vec<AnswerPoints>>,
        results: Vec<String>,
        results_images: Vec<String>
    ) -> Form {
        let id = U128((self.forms_by_id.len() + 1) as u128);

        let form: Form = Form {
            id: id,
            title: title,
            creation: env::block_timestamp(),
            questions: questions,
            possibly_answers: possibly_answers,
            answers_points: answer_points,
            results: results,
            results_images: results_images
        };
        self.save_form(form.clone());
        return form;
    }   
 }
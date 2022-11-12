use crate::*;

#[near_bindgen]
impl Contract {
    //Users create forms
    //The form will be created with mandatory questions, then N questions amount can be added 
    //Every question must be supported by its respectives possibly anwsers 
    //Each possibly answer must be supported by their points 
    //Once the form it's created, it will be save in two storages: {forms_by_creator} and {form_by_id}
    //For scalabilitie purposes the creator must attach a low NEAR amount, this will help to cover the form storage
    //then, once the storage is paid, the yoctoNEARs left will be refunded back
    //Dev to see how to query go to Enumerations module 
    //Requirements:
    // --- This function receives a deposit to cover the form storage
    // --- questions array lenght must match with possibly_answers lenght
    // --- possibly_answers matrix a its internal vectors lenght must match with answer_points input
    // --- results vector lenght must match with results_images
    #[payable]
    pub fn create_form(
        &mut self,
        title: String, //For identify the form
        //As seen in metadata the questions needs to be provided by a vector
        questions: Vec<String>,

        //Possibly answers for each questions, represented by a matrix where
        //every internal vector represents the answers for a given question
        possibly_answers: Vec<Vec<String>>,

        //Points for each answer, represented by a matrix
        answer_points: Vec<Vec<AnswerPoints>>,

        //Results will be shown as a vector, where the total points amount described in
        //answer_points will be divided by the results vector lenght
        results: Vec<String>,

        //Each image represent a result
        //The final result will correspond to the image index
        results_images: Vec<String>,
    ) -> Form {

        //Check if user is covering storage price
        assert!(env::attached_deposit() > 0, "Owling: Invalid deposit");    

        //Check if the title it's empty
        assert!(title != "", "Owling: Invalid title form");

        //Check if questions, answers and points have the same lenght
        //Ensures that each questions have its corresponded answers and every answer have its corresponded points
        assert!(
            questions.len() == possibly_answers.len()
                && possibly_answers.len() == answer_points.len(),
            "Owling: Invalid answer input"
        );

        //Check if results and images have the same lenght
        assert!(results.len() == results_images.len());

        //The form ID will be based on the total forms amount
        let id = U128((self.form_by_id.len() + 1) as u128);

        //Creator must attach an initial deposit for cover the form storages
        //Get initial storage usage by the form creation, before saved in storages
        let initial_storage_usage = env::storage_usage();

        //All forms must have the mandatory questions, in wich they doesn't represent a unique answer
        //the answer must be passed as an input, they also doesn't have answer points
        //All mandatory vectors must have the same lenght
        //Mandatory questions could be customizables
        let questions_mandatory = [
            "What is your aka".to_string(),
            "What is your age".to_string(),
            "Wich gender do you feel identified".to_string(),
        ];

        //Mandatory answers must be passed as input text by the frontend. This initialization it's by
        //reserve the index in the allocated vector memory, then this spaces will be filled by the FillForm method
        let answers_mandatory = [
            ["".to_string()].to_vec(),
            ["".to_string()].to_vec(),
            ["".to_string()].to_vec(),
        ];

        //Mndatory points are setted to 0, mandatory questions doesn't affects the final form result
        let answer_points_mandatory: Vec<Vec<u128>> =
            [[0].to_vec(), [0].to_vec(), [0].to_vec()].to_vec();

        //Create new vectors concatenating mandatory vectors with the input vectors
        //Dev see concat_strings, concat_array_strings and concat_array_u128 in Internal module
        let new_questions = concat_strings(&questions_mandatory, &questions);
        let new_possibly_answers = concat_array_strings(&answers_mandatory, &possibly_answers);
        let new_answers_points = concat_array_u128(&answer_points_mandatory, &answer_points);

        //Create the form object
        let form: Form = Form {
            id: id,
            title: title,
            //By default the status will be 0
            status: 0,
            creation: env::block_timestamp(),
            questions: new_questions,
            possibly_answers: new_possibly_answers,
            answers_points: new_answers_points,
            results: results,
            results_images: results_images,
        };

        //After the creation it is emitted a log for indexing the form created
        //Dev TheGraph Protocol
        env::log_str(
            &json!({
                "creator": env::signer_account_id().to_string(),
                "id": form.id.0.to_string(),
                "title": form.title.to_string(),
                "creation": form.creation.to_string(),
                "questions": form.questions,
                "possibly_answers": form.possibly_answers,
                "results": form.results,
            })
            .to_string(),
        );

        //Save the form in all storages
        //Dev see save_form in Internal module
        self.save_form(form.clone());

        //Calculate the final storage amount after save the form in storages
        //Then the creator deposit excedent will be refunded
        //Dev see refund_deposit in Internal module
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;
        refund_deposit(required_storage_in_bytes);

        return form;
    }

    pub fn submit_form (
        &mut self,
        form_id: FormId,
        anwsers: Vec<String>
    )  {

        //The form ID will be based on the total forms amount
        let id = U128((self.answer_by_id.len() + 1) as u128);

        let wallet_id = env::signer_account_id().to_string();

        let form = self.form_by_id(form_id).unwrap();

        
        
    }

}



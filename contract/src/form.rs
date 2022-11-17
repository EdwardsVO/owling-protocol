use crate::*;

#[near_bindgen]
impl Contract {
    // Users create forms
    // The form will be created with mandatory questions, then N questions amount can be added
    // Every question must be supported by its respectives possibly anwsers
    // Each possibly answer must be supported by their points
    // Once the form it's created, it will be save in two storages: {forms_by_creator} and {form_by_id}
    // For scalabilitie purposes the creator must attach a low NEAR amount, this will help to cover the form storage
    // then, once the storage is paid, the yoctoNEARs left will be refunded back
    // Dev to see how to query go to Enumerations module
    // Requirements:
    //  --- This function receives a deposit to cover the form storage
    //  --- questions array lenght must match with possibly_answers lenght
    //  --- possibly_answers matrix a its internal vectors lenght must match with answer_points input
    //  --- results vector lenght must match with results_images
    #[payable]
    pub fn create_form(
        &mut self,
        title: String, // For identify the form
        // As seen in metadata the questions needs to be provided by a vector
        questions: Vec<String>,

        // Possibly answers for each questions, represented by a matrix where
        // every internal vector represents the answers for a given question
        possibly_answers: Vec<Vec<String>>,

        // Points for each answer, represented by a matrix
        answer_points: Vec<Vec<AnswerPoints>>,

        // Results will be shown as a vector, where the total points amount described in
        // answer_points will be divided by the results vector lenght
        results: Vec<String>,

        // Each image represent a result
        // The final result will correspond to the image index
        results_images: Vec<String>,
    ) -> Form {
        // Check if user is covering storage price
        assert!(env::attached_deposit() > 0, "Owling: Invalid deposit");

        // Check if the title it's empty
        assert!(title != "", "Owling: Invalid title form");

        // Check if questions, answers and points have the same lenght
        // Ensures that each questions have its corresponded answers and every answer have its corresponded points
        assert!(
            questions.len() == possibly_answers.len()
                && possibly_answers.len() == answer_points.len(),
            "Owling: Invalid answer input"
        );

        // Check if results and images have the same lenght
        assert!(results.len() == results_images.len());

        // The form ID will be based on the total forms amount
        let id = U128((self.form_by_id.len() + 1) as u128);

        // Creator must attach an initial deposit for cover the form storages
        // Get initial storage usage by the form creation, before saved in storages
        let initial_storage_usage = env::storage_usage();

        // All forms must have the mandatory questions, in wich they doesn't represent a unique answer
        // the answer must be passed as an input, they also doesn't have answer points
        // All mandatory vectors must have the same lenght
        // Mandatory questions could be customizables
        let questions_mandatory = [
            "What is your aka".to_string(),
            "What is your age".to_string(),
            "Wich gender do you feel identified".to_string(),
        ];

        // Mandatory answers must be passed as input text by the frontend. This initialization it's by
        // reserve the index in the allocated vector memory, then this spaces will be filled by the FillForm method
        let answers_mandatory = [
            ["".to_string()].to_vec(),
            ["".to_string()].to_vec(),
            ["".to_string()].to_vec(),
        ];

        // Mndatory points are setted to 0, mandatory questions doesn't affects the final form result
        let answer_points_mandatory: Vec<Vec<u128>> =
            [[0].to_vec(), [0].to_vec(), [0].to_vec()].to_vec();

        // Create new vectors concatenating mandatory vectors with the input vectors
        // Dev see concat_strings, concat_array_strings and concat_array_u128 in Internal module
        let new_questions = concat_strings(&questions_mandatory, &questions);
        let new_possibly_answers = concat_array_strings(&answers_mandatory, &possibly_answers);
        let new_answers_points = concat_array_u128(&answer_points_mandatory, &answer_points);

        // Create the form object
        let form: Form = Form {
            id: id,
            title: title,
            // By default the status will be 0
            status: 0,
            creation: env::block_timestamp(),
            questions: new_questions,
            possibly_answers: new_possibly_answers,
            answers_points: new_answers_points,
            results: results,
            results_images: results_images,
        };

        // After the creation it is emitted a log for indexing the form created
        // Dev TheGraph Protocol
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

        // Save the form in all storages
        // Dev see save_form in Internal module
        self.save_form(form.clone());

        // Calculate the final storage amount after save the form in storages
        // Then the creator deposit excedent will be refunded
        // Dev see refund_deposit in Internal module
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;
        refund_deposit(required_storage_in_bytes);

        return form;
    }

    // Fill existent form
    // This method it's called by the user
    // The result will be calculed iterating over each answer and getting its answer points,
    // then this calculation will be scale in the next way:
    // The minimum points sum will be equivalent to zero points and the maximum points sum will be 100
    // Following the given proposition, the calculation will represent a percentage
    // After it's gotten the percentage, it's needed to calculate the final result that percentage represents
    // Finally it's match the final result with it respective image
    // Requirements:
    //  --- Form id must exist
    //  ---
    pub fn submit_form(&mut self, form_id: FormId, answers: Vec<String>) -> Answer {
        // Check if the form id exist
        let max_form_id = self.form_by_id.len() + 1;
        assert!(
            form_id > U128(0) || form_id <= U128(max_form_id as u128),
            "Owling: Invalid form_id"
        );
        
        //Check if the submitter isn't the creator
        let forms_created = self.forms_by_creator(env::signer_account_id());
        let form_not_exist = Some(forms_created.iter().filter(|form| form.id == form_id)).is_none(); 
        assert!(
            form_not_exist , "Owling: Creator can't answer their own form"
        );

        //Check if the submitter already fill the form
        let forms_submitted = self.answers_by_user(env::signer_account_id());
        let answer_not_exist = Some(forms_submitted.iter().filter(|ans| ans.form_id == form_id)).is_none();
        assert!(
            answer_not_exist, "Owling: Form already submitted"
        );
        
        // The form ID will be based on the total forms amount
        let id = ((self.answer_by_id.len() + 1) as u128);

        // get the function signer
        let wallet_id = env::signer_account_id();

        // Get the filled form
        let form = self.form_by_id(form_id).unwrap();

        // Initialize aux answer variables
        // This vars will contribute with the final result calculation
        let mut accumulate_points = 0;
        let mut max_points = 0;
        let mut min_points = 0;

        // Iterate over the form possibly answers, then it is compared the filled answers with the answers given 
        // Once it's match each answer with its possibility, the points of each one is obtained by its matrix indexes 
        for (i, row) in form.possibly_answers.iter().enumerate() {

            // Each possibly answer 
            for (j, col) in row.iter().enumerate() {

                // Compare each possibility with the given as input within the array
                if col == &answers[i] {

                    // Get the possibility answer points 
                    let accumulate = form.answers_points[i][j];

                    // Update the aux variable corresponded to the points the user obtained 
                    accumulate_points += accumulate;
                }
            }
        }

        // We must scale the {accumulate_points} variable, becuase we need to represent the min points sum as zero 
        // in the same way the max points sum as 100, this will help to perfectly get the asserted percentage
        // in relation with the form theme and purpose.
        // For scaling the accumulate points we need to get the minimum points results possibilibty 
        // Following that purpose we iterate each possibly points and we substract the min and the max value from each
        for (i, row) in form.answers_points.iter().enumerate() {

            // Update aux variables
            (max_points) += row.iter().max().unwrap_or(&0);
            (min_points) += row.iter().min().unwrap_or(&0);
        }

        //Get the form scale 0 - max_points representing 0% and 100%
        let scale = max_points - min_points;

        //Percentage of the {accumulate_points} in relation to the scale 
        let percentage = (accumulate_points - min_points) * 100 / scale;

        //Get the result index that represents the percentage
        let result_index = (((form.results.len() - 1) as u128) * percentage) / 100;

        //Get the final result given by the form results 
        let final_result = &form.results[(result_index as usize)];

        //Match the image and answer
        let final_result_image = &form.results_images[result_index as usize];

        // After the creation it is emitted a log for indexing the form created
        // Dev TheGraph Protocol
        env::log_str(
            &json!({
                "id": id.to_string(),
                "wallet_id": wallet_id.to_string(),
                "form_id": form_id.0.to_string(),
                "answers": answers,
                "total_points": percentage.to_string(),
                "final_result": final_result.to_string(),
                "final_image": final_result_image.to_string()
            })
            .to_string(),
        );

        //Answer object
        let answer = Answer {
            id: id,
            form_id: form.id,
            wallet_id: wallet_id,
            answers: answers,
            total_points: U128(percentage as u128),
            final_result: final_result.to_string(),
            final_image: final_result_image.to_string(),
        };

        //Save answers in both storages
        self.save_answer(answer.clone());

        return answer;
    }
}

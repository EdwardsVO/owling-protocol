# Owling Contract

Create forms with funny questions its possibly answers and results.

Fill the forms and share your result!
## Interacting with the contract

### Call functions

#### create_form (Parameters)

    Create Forms with the theme you want

Parameters:
  - title: String
  - questions: Array<Questions>
  - possibly_answers: Array<Array<String>>
  - answers_points: Array<Array<Number>>
  - results: Array<String>
  - results_images: Array<String>

#### submit_form (Parameters)

    Fill existent Forms and get your result and image

  Parameters:
    - form_id: Number,
    - answers: Array<String>

### View functions

#### forms_total_supply
    Return the total amount of forms created

#### form_by_id (Parameter)
    Get an especific form from its id

Parameter: 
  - from_id: Number

#### forms_by_creator (Parameter)
    Get all the forms created by one user
  
Parameter: 
  - creator_id: String


#### answer_by_id (Parameter)
    Get and answer by its id 

Parameter: 
  - answer_id: Number


#### answer_by_user (Parameter)
    Get all the forms submitted by one user

Parameter: 
  - user_id: String

## Call Example 


near call dev-1668044208565-60940875066684 create_form '{"title":"What kind of owl are you", "questions":["what time you feel more confortable to work", "Whats your favorite animal", "How pets do you currently have"], "possibly_answers":[["day", "night"], ["dog","cat","owl"], ["1", "2", "0"]], "answer_points":[[5,10],[2,3,100], [10,15,2]], "results":["Legendary Owl", "Commom Owl", "Rare Owl"], "results_images":["adsf.com","asdf.ve","asdf.com"]}' --account-id lexdev.testnet --deposit 0.1


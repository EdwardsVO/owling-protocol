# Owling Contract

## Form test example 


{
  id: '7',
  title: 'What kind of owl are you',
  status: 0,
  creation: 1668132125240224000,
  questions: [
    'What is your aka',
    'What is your age',
    'Wich gender do you feel identified',
    'what time you feel more confortable to work',
    'Whats your favorite animal',
    'How pets do you currently have'
  ],
  possibly_answers: [
    [ '' ],
    [ '' ],
    [ '' ],
    [ 'day', 'night' ],
    [ 'dog', 'cat', 'owl' ],
    [ '1', '2', '0' ]
  ],
  answers_points: [ [ 0 ], [ 0 ], [ 0 ], [ 5, 10 ], [ 2, 3, 100 ], [ 10, 15, 2 ] ],
  results: [ 'Legendary Owl', 'Commom Owl', 'Rare Owl' ],
  results_images: [ 'adsf.com', 'asdf.ve', 'adsfr.com' ]
}

## Call Example 


near call dev-1668044208565-60940875066684 create_form '{"title":"What kind of owl are you", "questions":["what time you feel more confortable to work", "Whats your favorite animal", "How pets do you currently have"], "possibly_answers":[["day", "night"], ["dog","cat","owl"], ["1", "2", "0"]], "answer_points":[[5,10],[2,3,100], [10,15,2]], "results":["Legendary Owl", "Commom Owl", "Rare Owl"], "results_images":["adsf.com","asdf.ve","asdf.com]}' --account-id lexdev.testnet --deposit 0.1


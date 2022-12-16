const express = require('express')
const app = express()
const port = 3000
const path = require('path')

//const cors = require('cors')
const fetch = (...args) =>
  import('node-fetch').then(({ default: fetch }) => fetch(...args));

app.use(express.json());

//app.use(cors());

app.use(express.static('../dist'));

app.get('/', function(req, res) {
  res.sendFile(path.join(__dirname, '../dist/index.html'));
});


app.get('/categories', (req, res) => {
  fetch('https://opentdb.com/api_category.php')
    .then(response => response.json())
    .then(json => res.send(json.trivia_categories))

})

app.get('/questions', (req, res) => {
  if (req.query.category)
    fetch('https://opentdb.com/api.php?amount=' + req.query.amount + '&category=' + req.query.category)
      .then(response => response.json())
      .then(needsmanip => {
        return needsmanip.results.map(item => {
          return {
            category: item.category,
            difficulty: item.difficulty,
            question: item.question,
            correct_answer: item.correct_answer,
            incorrect_answers: item.incorrect_answers
          }
        })
      })
      .then(json => res.send(json))
  else
    fetch('https://opentdb.com/api.php?amount=' + req.query.amount)
      .then(response => response.json())
      .then(json => res.send(json.results))
})

app.listen(port, () => {
  console.log(`Example app listening on port ${port}`)
})


module.exports = app;


use crate::components::Answers::Answers;
use crate::components::Front::Cat;
use crate::components::SafeHtml::SafeHtml;
use gloo_console::log;
use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen::JsValue;
use yew::events::MouseEvent;
use yew::prelude::*;
use yew::{function_component, html, Html, Properties};
use graphql_client::{GraphQLQuery, Response};
use std::error::Error;
use std::convert::TryInto;
use std::sync::Arc;
use crate::components::Quizbox::question_w_amountand_cat::QuestionWAmountandCatQuestionsByAmountAndCategoryId;


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/opentdb_schema.json",
    query_path = "graphql/questionswamountandcat.graphql",
    response_derives = "Debug, Clone"
)]
pub struct QuestionWAmountandCat;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub category: Cat,
    pub number: String,
    pub difficulty: String,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct Question {
    category: String,
    difficulty: String,
    question: String,
    correct_answer: String,
    incorrect_answers: Vec<String>,
}

#[function_component(Quizbox)]
pub fn quizbox(props: &Props) -> Html {
    let questions = use_state(|| {
        vec![QuestionWAmountandCatQuestionsByAmountAndCategoryId {
            category: "".to_string(),
            difficulty: "".to_string(),
            question: "".to_string(),
            correct_answer: "".to_string(),
            incorrect_answers: vec!["".to_string()],
        }]
    });
    let current_question = use_state(|| 0);
    let question_count = use_state(|| 0);
    let category_title = use_state(|| String::new());
    let disabled = use_state(|| false);
    let score = use_state(|| 0);
    let correct = use_state(|| "".to_string());
    let win = use_state(|| false);
    let next_text = use_state(|| "Next".to_string());

    

    //let questions_value = (*questions).clone();
    let disabled_value = (*disabled).clone();
    let score_value = (*score).clone();
    let correct_value = (*correct).clone();
    let current_question_value = (*current_question).clone();
    let question_count_value = (*question_count).clone();
    let category_title_value = (*category_title).clone();
    let next_text_value = (*next_text).clone();
    let win_value = (*win).clone();

    let disabled_callback = use_callback(
        move |disabled_val: bool, _| {
            disabled.set(disabled_val);
        },
        (),
    );

    let score_callback = use_callback(
        move |score_val: i32, _| {
            score.set(score_val);
        },
        (),
    );

    let correct_callback = use_callback(
        move |correct_val: String, _| {
            correct.clone().set(correct_val);
        },
        (),
    );


    let disabled_callback_comp = disabled_callback.clone();
    let correct_callback_comp = correct_callback.clone();

    let disabled_callback = disabled_callback.clone();
    let correct_callback = correct_callback.clone();
    let next_handler = Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        if current_question_value.clone() < question_count_value.clone() -1 {
          let cur_quest_num = current_question_value + 1;
          current_question.set(cur_quest_num);
        } else {
          win.set(true);
        }
        if current_question_value.clone() == question_count_value.clone() - 2 {
          next_text.set("Finish Quiz".to_string());
        }
        disabled_callback.emit(false);
        correct_callback.emit("unanswered".to_string());
    });

    fn capitalize_first_letter(s: &str) -> String {
      s[0..1].to_uppercase() + &s[1..]
    }

    {
        let questions = questions.clone();
        let question_count = question_count.clone();
        let question_number = props.number.clone();
        let cat_id = props.category.id.clone();
        let difficulty = props.difficulty.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let cat_id_int: i64 = cat_id.try_into().unwrap();
                    let question_number_int: i64 = question_number.parse().unwrap();
                    let request_body = QuestionWAmountandCat::build_query(question_w_amountand_cat::Variables {
                          amount: question_number_int,
                          category_id: cat_id_int,
                          difficulty: difficulty,
                    });
                    let response_body: Response<question_w_amountand_cat::ResponseData> = Request::post("/graphql")
                      .json(&request_body)
                          .unwrap()
                          .send()
                          .await
                          .unwrap()
                          .json::<Response<question_w_amountand_cat::ResponseData>>()
                          .await
                          .unwrap();
                    let fetched_questions = response_body.data.unwrap().questions_by_amount_and_category_id;
                    questions.set(fetched_questions.clone());
                    question_count.set(fetched_questions.len());
                    
                });
                || ()
            },
            (),
        )
    }

    
    html! (
      if !win_value.clone() { 
      <div class={classes!("bg-white", "max-w-lg", "rounded", "overflow-hidden", "shadow-lg")}>
        <div class={classes!("px-6", "py-4")}>
          <h3 class={classes!("bg-gray-100", "text-center", "py-3")}>
            {format!("Question #{} of {}", current_question_value + 1, question_count_value)}
            <br />
            {format!("Category: {}", props.category.name.clone())}
            <br />
            {format!("Difficulty: {}", capitalize_first_letter(&props.difficulty.clone()))}
          </h3>
          if correct_value.clone() == "correct" { 
            <div class={classes!("bg-green-100", "border-t", "border-b", "border-green-500", "text-green-700", "px-4", "py-3")} role={"alert"}>
              <p class={classes!("text-sm")}>{ "That answer is Correct!" }</p>
            </div>
          } else if correct_value.clone() == "incorrect" {
            <div class={classes!("bg-red-100", "border-t", "border-b", "border-red-500", "text-red-700", "px-4", "py-3")} role={"alert"}>
              <p class={classes!("text-sm")}>{ "That answer is Incorrect!" }</p>
            </div>
          }
          <div class={classes!("font-bold", "text-xl", "mb-2", "text-center", "py-4")}>
            <SafeHtml html={questions[current_question_value].question.clone()} />
          </div>
          <Answers
            incorrect_answers={questions[current_question_value].incorrect_answers.clone()}
            correct_answer={questions[current_question_value].correct_answer.clone()}
            disabled={disabled_value}
            set_disabled={disabled_callback_comp.clone()}
            set_score={score_callback.clone()}
            score={score_value}
            set_correct={correct_callback_comp.clone()}
            correct={correct_value} 
          />
        </div>
      
          if disabled_value {
            <div class={classes!("px-6", "pt-4", "pb-2", "text-center")}>
              <button class={classes!("bg-blue-500", "hover:bg-blue-700", "text-white", "font-bold", "py-2", "px-4", "rounded")} onclick={next_handler}>
                {next_text_value.clone()}
              </button>
            </div>
          }
     
      </div>
      }
      else {
        <div class={classes!("bg-white", "max-w-lg", "rounded", "overflow-hidden", "shadow-lg")}>
          <div class={classes!("px-6", "py-4")}>
            <h3 class={classes!("bg-gray-100", "text-center", "py-3", "px-8")}>
              { "Quiz Complete!" }
            </h3>
            <div class={classes!("font-bold", "text-xl", "mb-2", "text-center")}>
              {format!("Score: {} out of {} correct!", score_value.clone(), question_count_value.clone())}
              <h1 class={classes!("text-5xl")}>{format!("{}%", ((score_value.clone() as f32 / question_count_value.clone() as f32) * 100.0).round())}</h1>
            </div>
          </div>
        </div>
      }
    )
}

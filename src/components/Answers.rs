use yew::html::{IntoPropValue, IntoEventCallback};
use yew::prelude::*;
use gloo_console::log;
use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen::JsValue;
use yew::events::MouseEvent;
use yew::{html, AttrValue, Html, function_component, use_effect_with_deps, Properties};
use yew::virtual_dom::VNode;
use rand::prelude::*;
use crate::components::SafeHtml::SafeHtml;
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::borrow::Borrow;
use std::pin::Pin;
use web_sys::HtmlElement;
use web_sys::HtmlInputElement;
use web_sys::EventTarget;
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub incorrect_answers: Vec<String>,
    pub correct_answer: String,
    pub disabled: bool,
    //pub setDisabled: Callback<bool>,
    //pub setScore: Callback<i32>,
    pub score: i32,
    //pub setCorrect: Callback<String>,
    pub correct: String,
}


#[function_component(Answers)]
pub fn answers(props: &Props) -> Html {
    let answers_clicked = use_state(|| String::new());
    let answers = use_state(|| vec![]);
    let answers_value = (*answers).clone();
    let disabled = props.disabled.clone();

    let answer_handler = Callback::from(move |e: MouseEvent| {
        let target: EventTarget = e
            .target()
            .expect("Event should have a target when dispatched");
        e.prevent_default();
        
        let answer = target.unchecked_into::<HtmlInputElement>().value();
        let object = JsValue::from("world");
        web_sys::console::log_1(&answer.into());
        //web_sys::window().alert("You clicked an answer!"); 
        //answers_clicked.set(answer);
        //Pin::new(&mut disabled).set(true);
    });

      
    {
        let answers = answers.clone();
        let correct_answer = props.correct_answer.clone();
        let new_correct_answer = props.correct_answer.clone();
        let incorrect_answers = props.incorrect_answers.clone();
        let new_incorrect_answers = props.incorrect_answers.clone();
        use_effect_with_deps(
            move |_| {
                let mut answerss = vec![];
                answerss.push(new_correct_answer.clone());
                   for i in 0..new_incorrect_answers.len() {
                        answerss.push(new_incorrect_answers[i].clone());
                    }
                answerss.shuffle(&mut thread_rng());
                answers.set(answerss);
        }, correct_answer);

    }
    
    html! {
        <>  
            {
                answers.iter().map(|answer| {
                    html!{<button disabled={disabled} class={classes!("bg-blue-500", "m-0.5", "w-full", "hover:bg-blue-700", "text-white", "font-bold", "py-2", "px-4", "rounded")} value={answer.clone()} onclick={answer_handler.clone().into_event_callback()}/*onClick={(e: React.ChangeEvent<any>) => handleClick(e)}*/ >
                        <SafeHtml html={answer.clone()} />
                    </button>}
                }).collect::<Html>()
            }
        </>
    }
}

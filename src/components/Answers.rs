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
    pub set_disabled: Callback<bool>,
    pub set_score: Callback<i32>,
    pub score: i32,
    pub set_correct: Callback<String>,
    pub correct: String,
}


#[function_component(Answers)]
pub fn answers(props: &Props) -> Html {
    let answers_clicked = use_state(|| String::new());
    let answers = use_state(|| vec![]);
    let answers_value = (*answers).clone();
    let disabled = props.disabled.clone();

    let answers_clicked_value = (*answers_clicked).clone();

    let set_disabled = props.set_disabled.clone();
    let correct_answer_for_callback = props.correct_answer.clone();
    let correct_answer_for_answer = props.correct_answer.clone();
    let set_score = props.set_score.clone();
    let set_correct = props.set_correct.clone();
    let score = props.score.clone();
    let correct = props.correct.clone();
    let answer_handler = Callback::from(move |e: MouseEvent| {
        
        let target: EventTarget = e
            .target()
            .expect("Event should have a target when dispatched");
        e.prevent_default();
        
        let answer = target.unchecked_into::<HtmlInputElement>().value();
        let object = JsValue::from("world");
        web_sys::console::log_1(&answer.clone().into());
        //web_sys::window().alert("You clicked an answer!"); 
        answers_clicked.set(answer.clone());
        if answer == correct_answer_for_callback.clone() {
            //web_sys::window().expect("panic!").alert("You got it right!");
            let object = JsValue::from("You got it right!");
            web_sys::console::log_1(&object.into()); 
            set_score.emit(score + 1);
            set_correct.emit("correct".to_string());
        }
        else {
            //eb_sys::window().expect("panic!").alert("You got it wrong!");
            let object = JsValue::from("You got it wrong!");
            web_sys::console::log_1(&object.into());  
            set_correct.emit("incorrect".to_string());
        }
        //Pin::new(&mut disabled).set(true);
        set_disabled.emit(true);
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
                    html!{
                        if correct.clone() == "incorrect" && answer.clone() == correct_answer_for_answer.clone() {
                            <button disabled={disabled} class={classes!("bg-green-500", "m-0.5", "w-full", "text-white", "font-bold", "py-2", "px-4", "rounded")} value={answer.clone()}/*onClick={(e: React.ChangeEvent<any>) => handleClick(e)}*/ >
                                <SafeHtml html={answer.clone()} />
                            </button>
                        }
                        else if correct.clone() == "incorrect" && answer.clone() == answers_clicked_value.clone() {
                            <button disabled={disabled} class={classes!("bg-red-500", "m-0.5", "w-full", "text-white", "font-bold", "py-2", "px-4", "rounded")} value={answer.clone()}/*onClick={(e: React.ChangeEvent<any>) => handleClick(e)}*/ >
                                <SafeHtml html={answer.clone()} />
                            </button>
                        }
                        else if correct.clone() == "correct" && answer.clone() == correct_answer_for_answer.clone() {
                            <button disabled={disabled} class={classes!("bg-green-500", "m-0.5", "w-full", "text-white", "font-bold", "py-2", "px-4", "rounded")} value={answer.clone()}/*onClick={(e: React.ChangeEvent<any>) => handleClick(e)}*/ >
                                <SafeHtml html={answer.clone()} />
                            </button>
                        }
                        else if disabled {
                            <button disabled={disabled} class={classes!("bg-blue-500", "m-0.5", "w-full", "text-white", "font-bold", "py-2", "px-4", "rounded")} value={answer.clone()} >
                                <SafeHtml html={answer.clone()} />
                            </button>
                        } else {
                            <button disabled={disabled} class={classes!("bg-blue-500", "m-0.5", "w-full", "hover:bg-blue-700", "text-white", "font-bold", "py-2", "px-4", "rounded")} value={answer.clone()} onclick={answer_handler.clone().into_event_callback()}/*onClick={(e: React.ChangeEvent<any>) => handleClick(e)}*/ >
                                <SafeHtml html={answer.clone()} />
                            </button>
                        }
                    }
                }).collect::<Html>()
            }
        </>
    }
}

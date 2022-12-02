use gloo_console::log;
use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{EventTarget, HtmlInputElement};
use yew::events::SubmitEvent;
use yew::prelude::*;

use crate::components::Quizbox::Quizbox;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Cat {
    pub id: usize,
    pub name: String,
}

#[function_component(Front)]
pub fn front() -> Html {
    let category = use_state(|| vec![]);
    let startQuiz = use_state(|| false);
    let categoryPicked = use_state(|| Cat {
        id: 0,
        name: "Any Category".to_string(),
    });
    let numberQuestions = use_state(|| "10".to_string());

    let numberQuestionsValue = (*numberQuestions).clone();

    let startQuizValue = (*startQuiz).clone();

    let categoryPickedValue = (*categoryPicked).clone();

    let start_handler = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        startQuiz.set(true);
    });

    let category_handler = Callback::from(move |e: Event| {
        let target: EventTarget = e
            .target()
            .expect("Event should have a target when dispatched");
        // You must KNOW target is a HtmlInputElement, otherwise
        // the call to value would be Undefined Behaviour (UB).
        // Here we are sure that this is input element so we can convert it to the appropriate type without checking
        let cat_info = target.unchecked_into::<HtmlInputElement>().value();

        let mut cat_info = cat_info.split("_");
        let cat_info_vec = cat_info.collect::<Vec<&str>>();
        let cat_id = cat_info_vec[0];
        let cat_name = cat_info_vec[1];

        let value = Cat {
            id: cat_id.parse::<usize>().unwrap(),
            name: cat_name.to_string(),
        };
        categoryPicked.set(value);
    });

    let number_handler = Callback::from(move |e: Event| {
        let target: EventTarget = e
            .target()
            .expect("Event should have a target when dispatched");
        e.prevent_default();
        // You must KNOW target is a HtmlInputElement, otherwise
        // the call to value would be Undefined Behaviour (UB).
        // Here we are sure that this is input element so we can convert it to the appropriate type without checking
        numberQuestions.set(target.unchecked_into::<HtmlInputElement>().value());
    });

    {
        let category = category.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_categories: Vec<Cat> =
                        Request::get("http://localhost:3000/categories")
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    category.set(fetched_categories.clone());
                });
                || ()
            },
            (),
        )
    }

    html! {
        if !startQuizValue { <form class={classes!("w-full", "max-w-sm")} onsubmit={start_handler}>
        <div class={classes!("md:flex", "md:items-center", "mb-6")}>
          <div class={classes!("md:w-1/3")}>
            <label class={classes!("block", "text-gray-500", "font-bold", "md:text-right", "mb-1", "md:mb-0", "pr-4")} htmlFor="num-of-questions">
            { "No. of Questions" }
            </label>
          </div>
          <div class={classes!("md:w-2/3")}>
            <input class={classes!("bg-gray-200", "appearance-none", "border-2", "border-gray-200", "rounded", "w-full", "py-2", "px-4", "text-gray-700", "leading-tight", "focus:outline-none", "focus:bg-white", "focus:border-purple-500")} id="num-of-questions" type="number" value={numberQuestionsValue} min={5} max={30} onchange={number_handler} />
          </div>
        </div>
        <div class={classes!("md:flex", "md:items-center", "mb-6")}>
          <div class={classes!("md:w-1/3")}>
            <label class={classes!("block", "text-gray-500", "font-bold", "md:text-right", "mb-1", "md:mb-0", "pr-4")} htmlFor="categories">
            { "Quiz Category" }
            </label>
          </div>
          <div class={classes!("md:w-2/3")}>
            <select id="categories" class={classes!("block", "w-full", "bg-white", "border", "border-gray-400", "hover:border-gray-500", "px-4", "py-2", "pr-8", "rounded", "shadow", "leading-tight", "focus:outline-none", "focus:shadow-outline")} onchange={category_handler}>
              <option>{"Any Category"}</option>
              {
                category.iter().map(|cat| {
                    html!{<option key={cat.id.to_string()} value={cat.id.to_string() + "_" + &cat.name.clone()}>{ format!("{}", cat.name) }</option>}
                }).collect::<Html>()
              }
            </select>
          </div>
        </div>
        <div class={classes!("md:flex", "md:items-center")}>
          <div class={classes!("md:w-1/3")}></div>
          <div class={classes!("md:w-2/3")}>
            <button class={classes!("shadow", "bg-purple-500", "hover:bg-purple-400", "focus:shadow-outline", "focus:outline-none", "text-white", "font-bold", "py-2", "px-4", "rounded")} type="submit" >
            {"Start Quiz"}
            </button>
          </div>
        </div>
      </form>}
      else {
          <Quizbox category={categoryPickedValue} number={numberQuestionsValue} />
      }
    }
}

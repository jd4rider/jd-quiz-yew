use yew::prelude::*;

#[function_component(Answers)]
pub fn answers() -> Html {
    html! {
        <>
            <button /*disabled={disabled}*/ class={classes!("bg-blue-500", "m-0.5", "w-full", "hover:bg-blue-700", "text-white", "font-bold", "py-2", "px-4", "rounded")} /*key={index} value={answer} onClick={(e: React.ChangeEvent<any>) => handleClick(e)} dangerouslySetInnerHTML={{ __html: answer }}*/ >
                {" Here is a Answer "}
            </button>
            <button /*disabled={disabled}*/ class={classes!("bg-blue-500", "m-0.5", "w-full", "hover:bg-blue-700", "text-white", "font-bold", "py-2", "px-4", "rounded")} /*key={index} value={answer} onClick={(e: React.ChangeEvent<any>) => handleClick(e)} dangerouslySetInnerHTML={{ __html: answer }}*/ >
                {" Here is a Answer 2 "}
            </button>
            <button /*disabled={disabled}*/ class={classes!("bg-blue-500", "m-0.5", "w-full", "hover:bg-blue-700", "text-white", "font-bold", "py-2", "px-4", "rounded")} /*key={index} value={answer} onClick={(e: React.ChangeEvent<any>) => handleClick(e)} dangerouslySetInnerHTML={{ __html: answer }}*/ >
                {" Here is a Answer 3 "}
            </button>
            <button /*disabled={disabled}*/ class={classes!("bg-blue-500", "m-0.5", "w-full", "hover:bg-blue-700", "text-white", "font-bold", "py-2", "px-4", "rounded")} /*key={index} value={answer} onClick={(e: React.ChangeEvent<any>) => handleClick(e)} dangerouslySetInnerHTML={{ __html: answer }}*/ >
                {" Here is a Answer 4 "}
            </button>
        </>
    }
}

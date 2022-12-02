use yew::prelude::*;
mod components;

use crate::components::Front::Front;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <main class={classes!("flex", "min-h-screen", "flex-col", "items-center", "justify-center", "bg-gradient-to-b", "from-green-400", "to-blue-400")}>
                <Front />
            </main>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

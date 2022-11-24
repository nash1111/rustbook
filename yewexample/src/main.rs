use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
        <h1>{ "Hello World" }</h1>
        <h2>{ "HellowWorld2" }</h2>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
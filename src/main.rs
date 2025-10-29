use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let message = "Hello, Yew!";
    html! {
	<h1>{ message }</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

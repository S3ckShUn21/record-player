use yew::prelude::*;

// use yew::{function_component, html};

#[function_component(HelloWorld)]
fn hello_world() -> Html {
    html! { "This Text" }
}

fn main() {
    yew::start_app::<HelloWorld>();
}
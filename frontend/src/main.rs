use yew::prelude::*;

#[function_component(HelloWorld)]
fn hello_world() -> Html {

    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };

    html! {
        <div>
            <button {onclick}>{ "Increment value" }</button>
            <p>
                <b>{ "Current value: " }</b>
                { *counter }
            </p>
        </div>
    }
}

fn main() {
    yew::start_app::<HelloWorld>();
}
use yew::prelude::*;

mod dung;

#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(|| 0);
    let onclick = {
        let state = state.clone();
        Callback::from(move |_| state.set(*state + 1))
    };
    html! {
        <main>
            <h1>{ dung::hey().to_owned() + &dung::hi().to_string() }</h1>
            <h2>{ *state }</h2>
            <button onclick={onclick}>{ "Click" }</button>
        </main>
    }
}

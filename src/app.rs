use yew::prelude::*;

use crate::app::af::semantics::Semantics;
mod af;
mod sat;
use af::Attack;

#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(|| 0);
    let onclick = {
        let state = state.clone();
        Callback::from(move |_| state.set(*state + 1))
    };
    let framework = af::AF::new(vec![
        Attack::new(0, 1),
        Attack::new(1, 0),
        Attack::new(2, 3),
        Attack::new(3, 2),
        Attack::new(4, 4),
        Attack::new(4, 1),
        Attack::new(5, 0),
        Attack::new(5, 6),
        Attack::new(6, 7),
        Attack::new(7, 5)
    ]);
    html! {
        <main>
            <h2>{ *state }</h2>
            <button onclick={onclick}>{ "Click" }</button>
            <p>{ format!("AF: {:?}", framework) }</p>
            <p>{ format!("Complete: {:?}", framework.complete()) }</p>
        </main>
    }
}

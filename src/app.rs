use web_sys::{File, HtmlInputElement};
use yew::prelude::*;

use crate::app::af::semantics::Semantics;
mod af;
mod sat;
use af::Attack;

use self::af::AF;

fn default_af() -> AF {
    af::AF::new(vec![
        Attack::new(0, 1),
    ])
}

fn get_file_from_event(ev: Event) -> Option<File> {
    let input = ev.target_dyn_into::<HtmlInputElement>();
    if let Some(input) = input {
        if let Some(files) = input.files() {
            return files.get(0);
        }
    }
    None
}

#[function_component(App)]
pub fn app() -> Html {
    let framework = use_state(|| default_af());
    let load_af = {
        let framework = framework.clone();
        Callback::from(move |ev: Event| {
            let framework = framework.clone();
            if let Some(file) = get_file_from_event(ev) {
                wasm_bindgen_futures::spawn_local(async move {
                    let f = AF::from_file(file).await;
                    framework.set(f);
                });
            }
        })
    };
    html! {
        <main>
            <input type={"file"} onchange={load_af}/>
            <p>{ format!("AF: {:?}", framework) }</p>
            <p>{ format!("Complete: {:?}", framework.complete()) }</p>
            <p>{ format!("Stable: {:?}", framework.stable()) }</p>
            <p>{ format!("Preferred: {:?}", framework.preferred()) }</p>
        </main>
    }
}

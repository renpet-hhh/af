use web_sys::{File, HtmlInputElement};
use yew::prelude::*;

use crate::app::af::semantics::Semantics;
use crate::app::graph::VisDrawable;
mod af;
mod glue;
mod graph;
mod sat;
use af::Attack;

use self::af::AF;

fn default_af() -> AF {
    af::AF::new(vec![Attack(0, 1)])
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
    let vis_page = use_state(|| 0);
    let complete = framework.complete();
    let load_af = {
        let framework = framework.clone();
        let vis_page = vis_page.clone();
        Callback::from(move |ev: Event| {
            let framework = framework.clone();
            vis_page.set(0);
            if let Some(file) = get_file_from_event(ev) {
                wasm_bindgen_futures::spawn_local(async move {
                    let f = AF::from_file(file).await;
                    framework.set(f);
                });
            }
        })
    };
    let prev_page = {
        let vis_page = vis_page.clone();
        Callback::from(move |_| {
            if *vis_page > 0 {
                vis_page.set(*vis_page - 1);
            }
        })
    };
    let next_page = {
        let vis_page = vis_page.clone();
        let num_of_pages = complete.len();
        Callback::from(move |_| {
            if *vis_page < num_of_pages - 1 {
                vis_page.set(*vis_page + 1);
            }
        })
    };

    framework.update_vis("af-graph", complete.get(*vis_page));

    html! {
        <main>
            <input type={"file"} onchange={load_af}/>
            <button onclick={prev_page}>{ "Previous" }</button>
            <button onclick={next_page}>{ "Next" }</button>
            <div style="border: 2px solid black;width:512px;" id="af-graph"></div>
            <p>{ format!("AF: {:?}", framework) }</p>
            <p>{ format!("Complete: {:?}", complete) }</p>
            <p>{ format!("Stable: {:?}", framework.stable()) }</p>
            <p>{ format!("Preferred: {:?}", framework.preferred()) }</p>
        </main>
    }
}

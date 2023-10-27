
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

mod af;
mod glue;
mod graph;
mod sat;
pub(crate) mod util;

use crate::app::{
    af::{encoding::Enconding, semantics::Semantics, AF},
    graph::VisDrawable,
    util::read_file,
};


#[function_component(App)]
pub fn app() -> Html {
    let input_ref = use_node_ref();
    let textarea_ref = use_node_ref();
    let af_text_handle = use_state(|| String::from(""));
    let af_text = (*af_text_handle).clone();
    let parsed = Enconding::parse_simple(af_text.clone());
    let framework = AF::from(parsed);
    let vis_page = use_state(|| 0);
    let complete = framework.complete();

    let load_af = {
        let input_ref = input_ref.clone();
        let af_text_handle = af_text_handle.clone();
        let vis_page = vis_page.clone();
        Callback::from(move |_| {
            vis_page.set(0);
            // get the uploaded file
            let input: Option<HtmlInputElement> = input_ref.cast::<HtmlInputElement>();
            let file = input.and_then(|x| x.files()).and_then(|x| x.get(0));
            if let Some(file) = file {
                let af_text_handle = af_text_handle.clone();
                // asynchronously read the file and update the framework
                wasm_bindgen_futures::spawn_local(async move {
                    let text = read_file(file).await;
                    if let Some(text) = text {
                        af_text_handle.set(text);
                    }
                });
            }
        })
    };

    let handle_af_text_change = {
        let af_text_handle = af_text_handle.clone();
        let textarea_ref = textarea_ref.clone();
        Callback::from(move |_| {
            if let Some(textarea) = textarea_ref.cast::<HtmlTextAreaElement>() {
                af_text_handle.set(textarea.value());
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
            <input ref={input_ref} type={"file"} onchange={load_af}/>
            <textarea ref={textarea_ref} value={af_text} onchange={handle_af_text_change} />
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

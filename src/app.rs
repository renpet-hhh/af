use web_sys::{File, HtmlTextAreaElement};
use yew::prelude::*;

mod af;
mod components;
mod glue;
mod graph;
mod layout;
mod sat;
pub(crate) mod util;

use crate::app::{
    af::{encoding::Enconding, semantics::Semantics, AF},
    components::file_input::FileInput,
    components::preset::Presets,
    graph::VisDrawable,
    layout::Stack,
    util::read_file,
};

#[function_component(App)]
pub fn app() -> Html {
    let textarea_ref = use_node_ref();
    let af_text_handle = use_state(|| String::from(""));
    let af_text = (*af_text_handle).clone();
    let parsed = Enconding::parse_simple(af_text.clone());
    let framework = AF::from(parsed);
    let vis_page = use_state(|| 0);
    let complete = framework.complete();

    let load_af = {
        let af_text_handle = af_text_handle.clone();
        let vis_page = vis_page.clone();
        Callback::from(move |f: File| {
            vis_page.set(0);
            let af_text_handle = af_text_handle.clone();
            // asynchronously read the file and update the framework
            wasm_bindgen_futures::spawn_local(async move {
                let text = read_file(f).await;
                if let Some(text) = text {
                    af_text_handle.set(text);
                }
            });
        })
    };

    let handle_af_text_change = {
        let af_text_handle = af_text_handle.clone();
        let textarea_ref = textarea_ref.clone();
        Callback::from(move |_: Event| {
            if let Some(textarea) = textarea_ref.cast::<HtmlTextAreaElement>() {
                af_text_handle.set(textarea.value());
            }
        })
    };

    let prev_page = {
        let vis_page = vis_page.clone();
        Callback::from(move |_: MouseEvent| {
            if *vis_page > 0 {
                vis_page.set(*vis_page - 1);
            }
        })
    };

    let next_page = {
        let vis_page = vis_page.clone();
        let num_of_pages = complete.len();
        Callback::from(move |_: MouseEvent| {
            if *vis_page < num_of_pages - 1 {
                vis_page.set(*vis_page + 1);
            }
        })
    };

    let load_preset = {
        let vis_page = vis_page.clone();
        let af_text_handle = af_text_handle.clone();
        Callback::from(move |preset: &'static str| {
            vis_page.set(0);
            af_text_handle.set(preset.to_string());
        })
    };

    // Synchronize the network visualization
    framework.update_vis("af-graph", complete.get(*vis_page));

    html! {
        <Stack vertical={true}>
            <Stack align_items="center">
                <Stack vertical={true}>
                    <Stack align_items="center">
                        <FileInput id="load-af" text="Load AF" onload={load_af} />
                        <Presets onselect={load_preset} />
                    </Stack>
                    <textarea style="width: 512px; height: 512px;" ref={textarea_ref} value={af_text} onchange={handle_af_text_change} />
                </Stack>
                <Stack vertical={true}>
                    <Stack>
                        <button onclick={prev_page}>{ "Previous" }</button>
                        <button onclick={next_page}>{ "Next" }</button>
                    </Stack>
                    <div style="border: 2px solid black;width:512px;height:512px;" id="af-graph"></div>
                </Stack>
            </Stack>
            <p>{ format!("AF: {:?}", framework) }</p>
            <p>{ format!("Complete: {:?}", complete) }</p>
            <p>{ format!("Stable: {:?}", framework.stable()) }</p>
            <p>{ format!("Preferred: {:?}", framework.preferred()) }</p>
        </Stack>
    }
}

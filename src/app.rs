use wasm_bindgen::JsValue;
use web_sys::{File, HtmlTextAreaElement, console::log_1};
use yew::prelude::*;

mod af;
mod components;
mod glue;
mod graph;
mod sat;
mod util;

use crate::app::{
    af::{
        encoding::Enconding,
        semantics::{Semantics, SemanticsType},
        AF,
    },
    components::file_input::FileInput,
    components::{preset::Presets, select::Select},
    graph::VisDrawable,
    util::read_file,
};

#[function_component(App)]
pub fn app() -> Html {
    let textarea_ref = use_node_ref();
    let af_text_handle = use_state(|| String::from(""));
    let af_text = (*af_text_handle).clone();
    let parsed = Enconding::parse_simple(af_text.clone());
    let framework = AF::from(parsed);
    let semantics_type = use_state(|| SemanticsType::COMPLETE);
    let vis_page = use_state(|| 0);
    let semantics = framework.get_semantics(*semantics_type);
    

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
        let num_of_pages = semantics.len();
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
    framework.update_vis("af-graph", semantics.get(*vis_page));
    let flex_row = util::flex_row();
    let flex_col = util::flex_col();

    html! {
        <div class={classes!(flex_col.clone())}>
            <div class={classes!(flex_row.clone())}>
                <div class={classes!(flex_col.clone())}>
                    <div class={classes!(flex_row.clone())}>
                        <FileInput id="load-af" text="Load AF" onload={load_af} />
                        <Presets onselect={load_preset} />
                    </div>
                    <textarea class={
                        classes!("w-48", "h-64", "p-2", "border-2", "border-r-emerald-900", "border-solid", "resize-none")
                    } ref={textarea_ref} value={af_text} onchange={handle_af_text_change} />
                </div>
                <div class={classes!(flex_col)}>
                    <div class={classes!(flex_row.clone())}>
                        <i onclick={prev_page} class={classes!("fa-solid", "fa-arrow-left", "cursor-pointer")}></i>
                        <p>{ format!("{}/{}", 1 + *vis_page, semantics.len()) }</p>
                        <i onclick={next_page} class={classes!("fa-solid", "fa-arrow-right", "cursor-pointer")}></i>
                    </div>
                    <div style="border: 2px solid black;width:512px;height:512px;" id="af-graph"></div>
                    <div class={classes!(flex_row.clone())}>
                        <label>{ "Semantics:" }</label>
                        <Select<SemanticsType>
                            onchange={{
                                let semantics_type = semantics_type.clone();
                                Callback::from(move |s| semantics_type.set(s))
                            }}
                            current={*semantics_type}
                            options={vec![
                                SemanticsType::COMPLETE,
                                SemanticsType::PREFERRED,
                                SemanticsType::STABLE,
                            ]} />
                    </div>
                </div>
            </div>
        </div>
    }
}

use web_sys::{File, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FileInputProps {
    pub id: String,
    #[prop_or_default]
    pub text: String,
    pub onload: Callback<File>,
}

#[function_component]
pub fn FileInput(props: &FileInputProps) -> Html {
    let input_ref = use_node_ref();

    let handle_change = {
        let input_ref = input_ref.clone();
        let onload = props.onload.clone();
        Callback::from(move |_: Event| {
            // get first file from <input />
            let file = input_ref
                .cast::<HtmlInputElement>()
                .and_then(|x| x.files())
                .and_then(|x| x.get(0));
            if let Some(file) = file {
                onload.emit(file);
            }
        })
    };

    html! {
        <>
            <input
                id={props.id.clone()}
                ref={input_ref}
                type="file"
                onchange={handle_change}
                class={classes!("hidden")}
            />
            <label for={props.id.clone()}
                class={classes!("m-2", "p-3", "bg-emerald-700", "rounded", "text-stone-100", "cursor-pointer")}>{ props.text.clone() }</label>
        </>
    }
}

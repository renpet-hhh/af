use web_sys::HtmlSelectElement;
use yew::{function_component, use_node_ref, Callback, Html, Properties, html, classes};

#[derive(PartialEq, Properties)]
pub struct SelectProps<T: Into<String> + From<String> + std::cmp::PartialEq + Copy> {
    pub options: Vec<T>,
    pub onchange: Callback<T>,
    pub current: T,
}

#[function_component]
pub fn Select<T: Into<String> + From<String> + std::cmp::PartialEq + Copy + 'static>(
    props: &SelectProps<T>,
) -> Html {
    let select = use_node_ref();
    let handle_change = {
        let select = select.clone();
        let onchange = props.onchange.clone();
        Callback::from(move |_| {
            let node = select.cast::<HtmlSelectElement>();
            if let Some(node) = node {
                onchange.emit(T::from(node.value()));
            }
        })
    };
    if let Some(select) = select.cast::<HtmlSelectElement>() {
        select.set_value(&props.current.into());
    }
    html! {
        <select class={classes!("m-1", "p-1")} ref={select} onchange={handle_change}>
          {
            props.options.iter().map(|value| {
              let text: String = (*value).into();
              let selected = *value == props.current;
              html! {
                <option value={text.clone()} selected={selected}>{ text }</option>
              }
            }
            ).collect::<Html>()
          }
        </select>
    }
}

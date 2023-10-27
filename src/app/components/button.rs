use yew::function_component;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub children: Children,
    pub onclick: Callback<MouseEvent>
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    html! {
        <button style="
          text-decoration: none;
          border: none;
          background-color: #B4D2E7;
          font-size: 1.2rem;
          padding: 0.6rem;
          margin: 0.2rem;
          cursor: pointer;
        " onclick={props.onclick.clone()}>{ props.children.clone() }</button>
    }
}

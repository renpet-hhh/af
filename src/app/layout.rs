use yew::{function_component, html, Html, Properties, AttrValue};

#[derive(Properties, PartialEq)]
pub struct StackProps {
    #[prop_or(false)]
    pub vertical: bool,
    #[prop_or(AttrValue::from("stretch"))]
    pub align_items: AttrValue,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn Stack(props: &StackProps) -> Html {
    let flex_direction = match props.vertical {
        true => "column",
        false => "row",
    };
    html! {
      <div  style={format!("
            display: flex;
            flex-direction: {};
            align-items: {};
        ", flex_direction, props.align_items)}>
        { props.children.clone() }
      </div>
    }
}

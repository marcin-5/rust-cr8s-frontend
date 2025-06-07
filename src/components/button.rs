use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    /// Button text content
    pub children: Children,
    /// Optional click handler
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    /// Button type (submit, button, reset)
    #[prop_or("button".to_string())]
    pub button_type: String,
    /// CSS classes
    #[prop_or("btn btn-primary".to_string())]
    pub class: String,
    /// Whether the button is disabled
    #[prop_or(false)]
    pub disabled: bool,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let onclick = props.onclick.clone();

    let handle_click = Callback::from(move |e: MouseEvent| {
        if let Some(ref callback) = onclick {
            callback.emit(e);
        }
    });

    html! {
        <button
            type={props.button_type.clone()}
            class={props.class.clone()}
            disabled={props.disabled}
            onclick={handle_click}
        >
            {for props.children.iter()}
        </button>
    }
}

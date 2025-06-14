use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TextareaProps {
    pub label: AttrValue,
    pub name: AttrValue,
    pub value: AttrValue,
    pub onchange: Callback<Event>,
}

#[function_component(Textarea)]
pub fn textarea(props: &TextareaProps) -> Html {
    let html_id = format!("edit-{}", props.name);

    html! {
        <>
            <label for={html_id.clone()}>{props.label.clone()}</label>
            <textarea
                id={html_id}
                class="form-control"
                name={props.name.clone()}
                value={props.value.clone()}
                onchange={props.onchange.clone()}
            />
        </>
    }
}

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub item_name: String,
    pub item_id: i32,
    pub on_delete: Callback<MouseEvent>,
    pub on_cancel: Callback<MouseEvent>,
    pub is_loading: bool,
}

#[function_component(DeleteConfirmation)]
pub fn delete_confirmation(props: &Props) -> Html {
    html! {
        <div class="delete-confirmation">
            <p>
                {"Are you sure you want to delete "}
                {&props.item_name}
                {" #"}
                {props.item_id}
                {"?"}
            </p>
            <div class="button-group">
                <button
                    onclick={&props.on_delete}
                    class="btn btn-sm btn-danger me-2"
                    disabled={props.is_loading}
                >
                    if props.is_loading {
                        {"Deleting..."}
                    } else {
                        {"Delete"}
                    }
                </button>
                <button
                    onclick={&props.on_cancel}
                    class="btn btn-sm btn-secondary"
                    disabled={props.is_loading}
                >
                    {"Cancel"}
                </button>
            </div>
        </div>
    }
}

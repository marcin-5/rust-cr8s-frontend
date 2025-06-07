use web_sys::HtmlInputElement;
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;

use crate::api::rustaceans::api_rustacean_create;
use crate::components::alert::Alert;
use crate::components::input::Input;
use crate::contexts::CurrentUserContext;
use crate::Route;

#[function_component(RustaceanForm)]
pub fn rustacean_form() -> Html {
    let navigator = use_navigator().expect("Navigator not available");
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("The current user context is missing");

    let name_handle = use_state(String::default);
    let email_handle = use_state(String::default);
    let error_message_handle = use_state(String::default);

    let name = (*name_handle).clone();
    let email = (*email_handle).clone();
    let error_message = (*error_message_handle).clone();

    let name_changed = create_input_callback(name_handle.clone());
    let email_changed = create_input_callback(email_handle.clone());

    let onsubmit = create_submit_callback(
        name.clone(),
        email.clone(),
        error_message_handle.clone(),
        navigator.clone(),
        current_user_ctx.clone(),
    );

    html! {
        <form onsubmit={onsubmit}>
            if !error_message.is_empty() {
                <Alert alert_type={"danger"} message={error_message} />
            }
            <div class="mb-3">
                <Input
                    input_type="text"
                    name="name"
                    label="Name"
                    value={name}
                    onchange={name_changed}
                />
            </div>
            <div class="mb-3">
                <Input
                    input_type="email"
                    name="email"
                    label="E-mail"
                    value={email}
                    onchange={email_changed}
                />
            </div>
            <button type="submit" class="btn btn-primary">{"Save"}</button>
        </form>
    }
}

fn create_input_callback(handle: UseStateHandle<String>) -> Callback<Event> {
    Callback::from(move |e: Event| {
        if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
            handle.set(input.value());
        }
    })
}

fn create_submit_callback(
    name: String,
    email: String,
    error_handle: UseStateHandle<String>,
    navigator: Navigator,
    user_ctx: CurrentUserContext,
) -> Callback<SubmitEvent> {
    Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        handle_form_submission(
            name.clone(),
            email.clone(),
            error_handle.clone(),
            navigator.clone(),
            user_ctx.clone(),
        );
    })
}

fn handle_form_submission(
    name: String,
    email: String,
    error_handle: UseStateHandle<String>,
    navigator: Navigator,
    user_ctx: CurrentUserContext,
) {
    match &user_ctx.token {
        Some(token) => {
            let token = token.clone();
            spawn_local(async move {
                match api_rustacean_create(&token, name, email).await {
                    Ok(_) => navigator.push(&Route::Rustaceans),
                    Err(e) => error_handle.set(e.to_string()),
                }
            });
        }
        None => error_handle.set("Session expired. Please log in again".to_string()),
    }
}

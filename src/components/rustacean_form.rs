use web_sys::HtmlInputElement;
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;

use crate::api::rustaceans::{api_rustacean_create, api_rustacean_show, api_rustacean_update};
use crate::components::alert::Alert;
use crate::components::input::Input;
use crate::contexts::CurrentUserContext;
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub rustacean_id: Option<i32>,
}

#[function_component(RustaceanForm)]
pub fn rustacean_form(props: &Props) -> Html {
    let navigator = use_navigator().expect("Navigator not available");
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("The current user context is missing");

    let name_handle = use_state(String::default);
    let email_handle = use_state(String::default);
    let error_message_handle = use_state(String::default);
    let loading_handle = use_state(|| false);

    let name = (*name_handle).clone();
    let email = (*email_handle).clone();
    let error_message = (*error_message_handle).clone();
    let loading = *loading_handle;

    // Load existing rustacean data if editing
    {
        let name_handle = name_handle.clone();
        let email_handle = email_handle.clone();
        let error_message_handle = error_message_handle.clone();
        let loading_handle = loading_handle.clone();
        let current_user_ctx = current_user_ctx.clone();
        let rustacean_id = props.rustacean_id;

        use_effect_with(rustacean_id, move |&id| {
            if let Some(id) = id {
                if let Some(token) = &current_user_ctx.token {
                    loading_handle.set(true);
                    let token = token.clone();
                    let name_handle = name_handle.clone();
                    let email_handle = email_handle.clone();
                    let error_message_handle = error_message_handle.clone();
                    let loading_handle = loading_handle.clone();

                    spawn_local(async move {
                        match api_rustacean_show(&token, id).await {
                            Ok(rustacean) => {
                                name_handle.set(rustacean.name);
                                email_handle.set(rustacean.email);
                            }
                            Err(e) => error_message_handle.set(e.to_string()),
                        }
                        loading_handle.set(false);
                    });
                }
            }
        });
    }

    let name_changed = create_input_callback(name_handle.clone());
    let email_changed = create_input_callback(email_handle.clone());

    let onsubmit = create_submit_callback(
        name.clone(),
        email.clone(),
        error_message_handle.clone(),
        navigator.clone(),
        current_user_ctx.clone(),
        props.rustacean_id,
    );

    if loading {
        return html! {
            <div class="text-center">
                <div class="spinner-border" role="status">
                    <span class="visually-hidden">{"Loading..."}</span>
                </div>
            </div>
        };
    }

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
    rustacean_id: Option<i32>,
) -> Callback<SubmitEvent> {
    Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        handle_form_submission(
            name.clone(),
            email.clone(),
            error_handle.clone(),
            navigator.clone(),
            user_ctx.clone(),
            rustacean_id,
        );
    })
}

fn handle_form_submission(
    name: String,
    email: String,
    error_handle: UseStateHandle<String>,
    navigator: Navigator,
    user_ctx: CurrentUserContext,
    rustacean_id: Option<i32>,
) {
    match &user_ctx.token {
        Some(token) => {
            let token = token.clone();
            spawn_local(async move {
                let result = match rustacean_id {
                    Some(id) => api_rustacean_update(&token, id, name, email).await,
                    None => api_rustacean_create(&token, name, email).await,
                };

                match result {
                    Ok(_) => navigator.push(&Route::Rustaceans),
                    Err(e) => error_handle.set(e.to_string()),
                }
            });
        }
        None => error_handle.set("Session expired. Please log in again".to_string()),
    }
}

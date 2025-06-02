use crate::components::input::*;
use gloo_console::log;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Default, Clone)]
struct LoginCredentials {
    username: String,
    password: String,
}

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    let credentials = use_state(LoginCredentials::default);

    // Create a generic input change handler function
    let create_input_handler =
        |field_name: &'static str, credentials_handle: UseStateHandle<LoginCredentials>| {
            Callback::from(move |e: Event| {
                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                    let value = input.value();
                    credentials_handle.set(match field_name {
                        "username" => LoginCredentials {
                            username: value,
                            ..(*credentials_handle).clone()
                        },
                        "password" => LoginCredentials {
                            password: value,
                            ..(*credentials_handle).clone()
                        },
                        _ => (*credentials_handle).clone(),
                    });
                }
            })
        };

    // Create handlers for username and password inputs
    let username_changed = create_input_handler("username", credentials.clone());
    let password_changed = create_input_handler("password", credentials.clone());

    // Handle form submission
    let form_credentials = credentials.clone();
    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        let creds = (*form_credentials).clone();
        log!("Submitting form", creds.username, creds.password);
    });

    html! {
        <form onsubmit={onsubmit}>
            <div class="mb-3">
                <Input
                    input_type="text"
                    name="username"
                    label="Username"
                    value={(*credentials).username.clone()}
                    onchange={username_changed}
                />
            </div>
            <div class="mb-3">
                <Input
                    input_type="password"
                    name="password"
                    label="Password"
                    value={(*credentials).password.clone()}
                    onchange={password_changed}
                />
            </div>
            <button type="submit" class="btn btn-primary">{"Login"}</button>
        </form>
    }
}

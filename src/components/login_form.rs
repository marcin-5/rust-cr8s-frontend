use crate::api::user::{api_login, api_me, LoginResponse, MeResponse};
use crate::components::alert::*;
use crate::components::input::*;
use gloo_console::log;
use web_sys::HtmlInputElement;
use yew::platform::spawn_local;
use yew::prelude::*;

#[derive(Default, Clone)]
struct LoginCredentials {
    username: String,
    password: String,
}

fn display_error(err: &gloo_net::Error) -> String {
    match err {
        gloo_net::Error::JsError(e) => format!("Network error: {}", e),
        gloo_net::Error::SerdeError(e) => format!("Data parsing error: {}", e),
        _ => "Unknown error occurred".to_string(),
    }
}

async fn login(
    username: String,
    password: String,
) -> Result<(LoginResponse, MeResponse), gloo_net::Error> {
    let login_response = api_login(username, password).await?;
    let me_response = api_me(&login_response.token).await?;
    Ok((login_response, me_response))
}

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    let credentials = use_state(LoginCredentials::default);
    let error_message_handle = use_state(String::default);
    let error_message = (*error_message_handle).clone();

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
    let cloned_error_handle = error_message_handle.clone();
    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        let creds = (*form_credentials).clone();
        let error_handle = cloned_error_handle.clone();
        spawn_local(async move {
            match login(creds.username, creds.password).await {
                Ok(responses) => log!(responses.1.username),
                Err(e) => error_handle.set(display_error(&e)),
            }
        });
    });

    html! {
        <form onsubmit={onsubmit}>
            if !error_message.is_empty() {
                <Alert alert_type={"danger"} message={error_message} />
            }
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

use crate::api::user::{api_login, api_me, LoginResponse, User};
use crate::components::alert::*;
use crate::components::button::Button;
use crate::components::input::*;
use crate::contexts::{CurrentUserAction, CurrentUserContext};
use crate::Route;
use web_sys::HtmlInputElement;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Default, Clone)]
struct LoginCredentials {
    username: String,
    password: String,
}

fn display_error(err: &gloo_net::Error) -> String {
    match err {
        gloo_net::Error::GlooError(e) => {
            if e.contains("Invalid credentials") {
                "Invalid username or password".to_string()
            } else {
                format!("Error: {}", e)
            }
        }
        gloo_net::Error::JsError(e) => format!("Network error: {}", e),
        gloo_net::Error::SerdeError(e) => format!("Data parsing error: {}", e),
    }
}

async fn login(
    username: String,
    password: String,
) -> Result<(LoginResponse, User), gloo_net::Error> {
    let login_response = api_login(username, password).await?;
    let user = api_me(&login_response.token).await?;
    Ok((login_response, user))
}

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    let navigator = use_navigator().expect("Navigator not available");
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("The current user context is missing");
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
        let cloned_navigator = navigator.clone();
        let cloned_current_user_ctx = current_user_ctx.clone();
        spawn_local(async move {
            match login(creds.username, creds.password).await {
                Ok((login_response, user)) => {
                    cloned_current_user_ctx.dispatch(CurrentUserAction::LoginSuccess {
                        token: login_response.token,
                        user,
                    });
                    cloned_navigator.push(&Route::Home);
                }
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
            <Button button_type="submit" class="btn btn-success">{"Login"}</Button>
        </form>
    }
}

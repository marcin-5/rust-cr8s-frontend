use crate::api::rustaceans::api_rustacean_delete;
use crate::components::alert::Alert;
use crate::components::button::Button;
use crate::components::header::Header;
use crate::components::sidebar::Sidebar;
use crate::contexts::CurrentUserContext;
use crate::Route;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: i32,
}

fn handle_delete_rustacean(
    token: String,
    id: i32,
    navigator: Navigator,
    error_handle: UseStateHandle<String>,
) {
    spawn_local(async move {
        match api_rustacean_delete(&token, id).await {
            Ok(()) => navigator.push(&Route::Rustaceans),
            Err(e) => error_handle.set(e.to_string()),
        }
    });
}

#[function_component(RustaceansDelete)]
pub fn rustaceans_delete(props: &Props) -> Html {
    let navigator = use_navigator().expect("Navigator not available");
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("Current user context is missing");
    let error_message_handle = use_state(String::default);
    let error_message = (*error_message_handle).clone();

    match &current_user_ctx.token {
        Some(token) => {
            let token = token.clone();
            let rustacean_id = props.id;
            let onclick = Callback::from(move |e: MouseEvent| {
                e.prevent_default();
                handle_delete_rustacean(
                    token.clone(),
                    rustacean_id,
                    navigator.clone(),
                    error_message_handle.clone(),
                );
            });

            html! {
                <div class="container">
                    <div class="row">
                        <div class="col-sm-auto">
                            <Sidebar />
                        </div>
                        <div class="col mt-3">
                            <Header />
                            if !error_message.is_empty() {
                                <Alert alert_type={"danger"} message={error_message} />
                            }
                            <p>
                                {"Are you sure you want to delete rustacean #"}
                                {rustacean_id}{"?"}
                            </p>
                            <Button onclick={onclick} class="btn btn-danger">{"Delete"}</Button>
                        </div>
                    </div>
                </div>
            }
        }
        None => html! {
            <Redirect<Route> to={Route::Login} />
        },
    }
}

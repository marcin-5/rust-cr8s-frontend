use crate::api::crates::api_crate_delete;
use crate::components::alert::Alert;
use crate::components::delete_confirmation::DeleteConfirmation;
use crate::contexts::CurrentUserContext;
use crate::pages::common::layout::AuthenticatedLayout;
use crate::Route;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub crate_id: i32,
}

#[function_component(CratesDelete)]
pub fn crates_delete(props: &Props) -> Html {
    let navigator = use_navigator().expect("Navigator not available");
    let current_user_ctx = use_context::<CurrentUserContext>();
    let error_message_handle = use_state(String::default);
    let is_loading_handle = use_state(|| false);

    let error_message = (*error_message_handle).clone();
    let is_loading = *is_loading_handle;

    // Check if user is authenticated
    let current_user_ctx = match current_user_ctx {
        Some(ctx) => ctx,
        None => {
            // Redirect to login if no context
            navigator.push(&Route::Login);
            return html! {};
        }
    };

    let on_delete = {
        let crate_id = props.crate_id;
        let token = current_user_ctx.token.clone();
        let navigator = navigator.clone();
        let error_handle = error_message_handle.clone();
        let loading_handle = is_loading_handle.clone();

        Callback::from(move |_: MouseEvent| {
            // Check if token exists
            let token = match &token {
                Some(t) if !t.is_empty() => t.clone(),
                _ => {
                    error_handle.set("Authentication required".to_string());
                    return;
                }
            };

            let navigator = navigator.clone();
            let error_handle = error_handle.clone();
            let loading_handle = loading_handle.clone();

            loading_handle.set(true);
            error_handle.set(String::new()); // Clear previous errors

            spawn_local(async move {
                match api_crate_delete(&token, crate_id).await {
                    Ok(()) => navigator.push(&Route::Crates),
                    Err(e) => {
                        error_handle.set(e.to_string());
                        loading_handle.set(false);
                    }
                }
            });
        })
    };

    let on_cancel = {
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| {
            navigator.push(&Route::Crates);
        })
    };

    html! {
        <AuthenticatedLayout>
            if !error_message.is_empty() {
                <Alert alert_type={"danger"} message={error_message} />
            }
            <DeleteConfirmation
                item_name={"crate"}
                item_id={props.crate_id}
                on_delete={on_delete}
                on_cancel={on_cancel}
                is_loading={is_loading}
            />
        </AuthenticatedLayout>
    }
}

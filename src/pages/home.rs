use crate::{contexts::CurrentUserContext, Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("Current user context is missing");
    match &current_user_ctx.user {
        Some(user) => {
            html! {
                <div class="container text-center">
                    {"Welcome user "}{user.username.clone()}
                </div>
            }
        }
        None => html! {
            <Redirect<Route> to={Route::Login} />
        },
    }
}

use crate::components::sidebar::Sidebar;
use crate::{contexts::CurrentUserContext, Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("The current user context is missing");
    match &current_user_ctx.user {
        Some(user) => {
            html! {
                <div class="container">
                    <div class="row">
                        <div class="col">
                            <Sidebar />
                        </div>
                        <div class="col">
                            {"Welcome user "}{user.username.clone()}
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

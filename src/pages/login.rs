use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::login_form::LoginForm;
use crate::contexts::CurrentUserContext;
use crate::Route;

#[function_component(Login)]
pub fn login() -> Html {
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("Current user context is missing");

    if current_user_ctx.is_loading {
        return html! {
            <div class="container">
                <div class="row min-vh-100 justify-content-center align-items-center">
                    <div class="col-md-4 text-center">
                        <div class="spinner-border" role="status">
                            <span class="visually-hidden">{"Loading..."}</span>
                        </div>
                    </div>
                </div>
            </div>
        };
    }

    match &current_user_ctx.user {
        Some(_) => html! {
            <Redirect<Route> to={Route::Home} />
        },
        None => html! {
            <div class="container">
                <div class="row min-vh-100 justify-content-center align-items-center">
                    <div class="col-md-4">
                        <LoginForm />
                    </div>
                </div>
            </div>
        },
    }
}

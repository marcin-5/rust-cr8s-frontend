use crate::components::button::Button;
use crate::{
    contexts::{CurrentUserAction, CurrentUserContext},
    Route,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("The current user context is missing");

    if current_user_ctx.is_loading {
        return html! {
            <div class="text-center">
                <div class="spinner-border" role="status">
                    <span class="visually-hidden">{"Loading..."}</span>
                </div>
            </div>
        };
    }

    match &current_user_ctx.user {
        Some(user) => {
            let cloned_user_ctx = current_user_ctx.clone();
            let onclick = Callback::from(move |e: MouseEvent| {
                e.prevent_default();
                cloned_user_ctx.dispatch(CurrentUserAction::LoginFail);
            });
            html! {
                <div class="text-end">
                    <p>
                        <span class="pe-1">{"Welcome "}{user.username.clone()}</span>
                        <Button class="btn btn-outline-danger" onclick={onclick}>{"Logout"}</Button>
                    </p>
                </div>
            }
        }
        None => html! {
            <Redirect<Route> to={Route::Login} />
        },
    }
}

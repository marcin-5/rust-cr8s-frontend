use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::header::Header;
use crate::components::sidebar::Sidebar;
use crate::contexts::CurrentUserContext;
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(AuthenticatedLayout)]
pub fn authenticated_layout(props: &Props) -> Html {
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("The current user context is missing");

    match &current_user_ctx.token {
        Some(_token) => {
            html! {
                <div class="container">
                    <div class="row">
                        <div class="col-sm-auto">
                            <Sidebar />
                        </div>
                        <div class="col mt-3">
                            <Header />
                            { for props.children.iter() }
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

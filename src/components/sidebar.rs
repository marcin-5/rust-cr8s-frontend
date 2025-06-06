use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let current_route = use_route::<Route>().expect("No current route defined");

    const BASE_NAV_CLASS: &str = "nav-link";

    let get_link_classes = |route: Route| -> Classes {
        if current_route == route {
            classes!(BASE_NAV_CLASS, "active")
        } else {
            classes!(BASE_NAV_CLASS)
        }
    };

    html! {
        <nav class="navbar navbar-light">
            <ul class="nav navbar-nav">
                <li class="nav-item">
                    <Link<Route> to={Route::Home} classes={get_link_classes(Route::Home)}>
                        {"Home"}
                    </Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={Route::Rustaceans} classes={get_link_classes(Route::Rustaceans)}>
                        {"Rustaceans"}
                    </Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={Route::Crates} classes={get_link_classes(Route::Crates)}>
                        {"Crates"}
                    </Link<Route>>
                </li>
            </ul>
        </nav>
    }
}

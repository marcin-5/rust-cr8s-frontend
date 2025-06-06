use yew::prelude::*;
use yew_router::prelude::*;

mod api;
mod components;
mod contexts;
mod pages;

#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[at("/")]
    Home,
    #[at("/rustaceans")]
    Rustaceans,
    #[at("/crates")]
    Crates,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Login => html! { <pages::login::Login /> },
        Route::Home => html! { <pages::home::Home /> },
        _ => html! { <pages::not_found::NotFound /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <contexts::CurrentUserProvider>
                <Switch<Route> render ={ switch } />
            </contexts::CurrentUserProvider>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

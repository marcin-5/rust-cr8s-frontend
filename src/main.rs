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
    #[at("/rustaceans/add")]
    RustaceansAdd,
    #[at("/rustaceans/:id/delete")]
    RustaceansDelete { id: i32 },
    #[at("/rustaceans/:id/edit")]
    RustaceansEdit { id: i32 },
    #[at("/crates")]
    Crates,
    #[at("/crates/add")]
    CratesAdd,
    #[at("/crates/:id/delete")]
    CratesDelete { id: i32 },
    #[at("/crates/:id/edit")]
    CratesEdit { id: i32 },
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
        Route::Rustaceans => html! { <pages::rustaceans::index::Rustaceans /> },
        Route::RustaceansAdd => html! { <pages::rustaceans::add::RustaceansAdd /> },
        Route::RustaceansDelete { id } => {
            html! { <pages::rustaceans::delete::RustaceansDelete {id} /> }
        }

        Route::RustaceansEdit { id } => html! { <pages::rustaceans::edit::RustaceansEdit {id} /> },
        Route::Crates => html! { <pages::crates::index::Crates /> },
        Route::CratesAdd => html! { <pages::crates::add::CratesAdd /> },
        Route::CratesEdit { id } => html! { <pages::crates::edit::CratesEdit crate_id={id} /> },
        Route::CratesDelete { id } => {
            return html! { <pages::crates::delete::CratesDelete crate_id={id} /> };
        }
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
    dotenv::dotenv().ok();

    yew::Renderer::<App>::new().render();
}

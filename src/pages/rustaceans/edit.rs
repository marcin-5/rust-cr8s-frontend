use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::header::Header;
use crate::components::rustacean_form::RustaceanForm;
use crate::components::sidebar::Sidebar;
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: i32,
}

#[function_component(RustaceansEdit)]
pub fn rustaceans_edit(props: &Props) -> Html {
    html! {
        <div class="container">
            <div class="row">
                <div class="col-sm-auto">
                    <Sidebar />
                </div>
                <div class="col mt-3">
                    <Header />
                    <h1>{"Edit Rustacean"}</h1>
                    <RustaceanForm rustacean_id={Some(props.id)} />
                </div>
            </div>
        </div>
    }
}

use yew::prelude::*;

use crate::components::rustacean_form::RustaceanForm;
use crate::pages::common::layout::AuthenticatedLayout;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: i32,
}

#[function_component(RustaceansEdit)]
pub fn rustaceans_edit(props: &Props) -> Html {
    html! {
        <AuthenticatedLayout>
            <h1>{"Edit Rustacean"}</h1>
            <RustaceanForm rustacean_id={Some(props.id)} />
        </AuthenticatedLayout>
    }
}

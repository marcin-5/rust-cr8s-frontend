use yew::prelude::*;

use crate::components::rustacean_form::RustaceanForm;
use crate::pages::common::layout::AuthenticatedLayout;

#[function_component(RustaceansAdd)]
pub fn rustaceans_add() -> Html {
    html! {
        <AuthenticatedLayout>
            <RustaceanForm rustacean_id={None} />
        </AuthenticatedLayout>
    }
}

use yew::prelude::*;

use crate::components::rustacean_list::RustaceanList;
use crate::pages::common::layout::AuthenticatedLayout;

#[function_component(Rustaceans)]
pub fn rustaceans() -> Html {
    html! {
        <AuthenticatedLayout>
            <RustaceanList />
        </AuthenticatedLayout>
    }
}

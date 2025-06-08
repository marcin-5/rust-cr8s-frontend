use yew::prelude::*;

use crate::components::crate_list::CrateList;
use crate::pages::common::layout::AuthenticatedLayout;

#[function_component(Crates)]
pub fn crates() -> Html {
    html! {
        <AuthenticatedLayout>
            <CrateList />
        </AuthenticatedLayout>
    }
}

use yew::prelude::*;

use crate::components::crate_form::CrateForm;
use crate::pages::common::layout::AuthenticatedLayout;

#[function_component(CratesAdd)]
pub fn crates_add() -> Html {
    html! {
        <AuthenticatedLayout>
            <CrateForm crate_id={None} />
        </AuthenticatedLayout>
    }
}

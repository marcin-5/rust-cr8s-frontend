use yew::prelude::*;

use crate::components::crate_form::CrateForm;
use crate::pages::common::layout::AuthenticatedLayout;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub crate_id: i32,
}

#[function_component(CratesEdit)]
pub fn crates_edit(props: &Props) -> Html {
    html! {
        <AuthenticatedLayout>
            <CrateForm crate_id={Some(props.crate_id)} />
        </AuthenticatedLayout>
    }
}

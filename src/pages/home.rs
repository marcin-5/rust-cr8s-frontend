use crate::pages::common::layout::AuthenticatedLayout;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <AuthenticatedLayout>
            {"Have a great day!"}
        </AuthenticatedLayout>
    }
}

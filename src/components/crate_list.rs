use crate::{
    api::crates::api_crates, components::button::Button, contexts::CurrentUserContext, Route,
};
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;

#[function_component(CrateList)]
pub fn crate_list() -> Html {
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("The current user context is missing");
    let navigator = use_navigator().expect("Navigator not available");
    let crates_handle = use_state(|| vec![]);
    let crates = (*crates_handle).clone();

    // Use use_effect to fetch data only once when the component mounts or token changes
    {
        let crates_handle = crates_handle.clone();
        let token = current_user_ctx.token.clone();
        use_effect_with(token, move |token| {
            if let Some(token) = token {
                let cloned_token = token.clone();
                let crates_handle = crates_handle.clone();
                spawn_local(async move {
                    let response = api_crates(&cloned_token).await.unwrap();
                    crates_handle.set(response);
                });
            }
            || ()
        });
    }

    // Check if the context is still loading
    if current_user_ctx.is_loading {
        return html! { <></> };
    }

    match &current_user_ctx.token {
        Some(_token) => {
            let navigator_clone = navigator.clone();
            let add_crate_click = Callback::from(move |_: MouseEvent| {
                navigator_clone.push(&Route::CratesAdd);
            });

            html! {
                <>
                    <div class="mb-3">
                        <Button onclick={add_crate_click} class="btn btn-success">
                            {"+ Add new crate"}
                        </Button>
                    </div>
                    <table class="table">
                        <thead>
                            <th>{"ID"}</th>
                            <th>{"Code"}</th>
                            <th>{"Name"}</th>
                            <th>{"Rustacean ID"}</th>
                            <th>{"Version"}</th>
                            <th>{"Description"}</th>
                            <th>{"Created at"}</th>
                            <th>{"Operations"}</th>
                        </thead>
                        <tbody>
                            {
                                crates.into_iter().map(|a_crate| {
                                    let navigator_edit = navigator.clone();
                                    let navigator_delete = navigator.clone();
                                    let crate_id = a_crate.id;

                                    let edit_click = Callback::from(move |_: MouseEvent| {
                                        navigator_edit.push(&Route::CratesEdit { id: crate_id });
                                    });

                                    let delete_click = Callback::from(move |_: MouseEvent| {
                                        navigator_delete.push(&Route::CratesDelete { id: crate_id });
                                    });

                                    html! {
                                        <tr>
                                            <td>{a_crate.id}</td>
                                            <td>{a_crate.code}</td>
                                            <td>{a_crate.name}</td>
                                            <td>{a_crate.rustacean_id}</td>
                                            <td>{a_crate.version}</td>
                                            <td>{a_crate.description}</td>
                                            <td>{a_crate.created_at}</td>
                                            <td>
                                                <Button onclick={edit_click} class="btn btn-sm btn-outline-secondary me-2">
                                                    {"Edit"}
                                                </Button>
                                                <Button onclick={delete_click} class="btn btn-sm btn-outline-danger">
                                                    {"Delete"}
                                                </Button>
                                            </td>
                                        </tr>
                                    }
                                }).collect::<Html>()
                            }
                        </tbody>
                    </table>
                </>
            }
        }
        None => {
            html! {
                <Redirect<Route> to={Route::Login} />
            }
        }
    }
}

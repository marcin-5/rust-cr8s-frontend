use crate::{
    api::rustaceans::api_rustaceans, components::button::Button, contexts::CurrentUserContext,
    Route,
};
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;

#[function_component(RustaceanList)]
pub fn rustacean_list() -> Html {
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("The current user context is missing");
    let navigator = use_navigator().expect("Navigator not available");
    let rustaceans_handle = use_state(|| vec![]);
    let rustaceans = (*rustaceans_handle).clone();

    // Use use_effect to fetch data only once when the component mounts or token changes
    {
        let rustaceans_handle = rustaceans_handle.clone();
        let token = current_user_ctx.token.clone();
        use_effect_with(token, move |token| {
            if let Some(token) = token {
                let cloned_token = token.clone();
                let rustaceans_handle = rustaceans_handle.clone();
                spawn_local(async move {
                    let response = api_rustaceans(&cloned_token).await.unwrap();
                    rustaceans_handle.set(response);
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
            let add_rustacean_click = Callback::from(move |_: MouseEvent| {
                navigator_clone.push(&Route::RustaceansAdd);
            });

            html! {
                <>
                    <div class="mb-3">
                        <Button onclick={add_rustacean_click} class="btn btn-success">
                            {"+ Add new rustacean"}
                        </Button>
                    </div>
                    <table class="table">
                        <thead>
                            <th>{"ID"}</th>
                            <th>{"Name"}</th>
                            <th>{"Email"}</th>
                            <th>{"Created at"}</th>
                            <th>{"Operations"}</th>
                        </thead>
                        <tbody>
                            {
                                rustaceans.into_iter().map(|rustacean| {
                                    let navigator_edit = navigator.clone();
                                    let navigator_delete = navigator.clone();
                                    let rustacean_id = rustacean.id;

                                    let edit_click = Callback::from(move |_: MouseEvent| {
                                        navigator_edit.push(&Route::RustaceansEdit { id: rustacean_id });
                                    });

                                    let delete_click = Callback::from(move |_: MouseEvent| {
                                        navigator_delete.push(&Route::RustaceansDelete { id: rustacean_id });
                                    });

                                    html! {
                                        <tr>
                                            <td>{rustacean.id}</td>
                                            <td>{rustacean.name}</td>
                                            <td>{rustacean.email}</td>
                                            <td>{rustacean.created_at}</td>
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

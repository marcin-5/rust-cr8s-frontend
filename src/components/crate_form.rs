use crate::api::crates::{api_crate_create, api_crate_show, api_crate_update, Crate};
use crate::api::rustaceans::api_rustaceans;
use crate::components::alert::Alert;
use crate::components::button::Button;
use crate::components::input::Input;
use crate::components::select::Select;
use crate::components::textarea::Textarea;
use crate::contexts::CurrentUserContext;
use crate::Route;
use web_sys::{HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub crate_id: Option<i32>,
}

#[function_component(CrateForm)]
pub fn crate_form(props: &Props) -> Html {
    let navigator = use_navigator().expect("Navigator not available");
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("Current user context is missing");

    let name_handle = use_state(String::default);
    let code_handle = use_state(String::default);
    let rustacean_id_handle = use_state(String::default);
    let version_handle = use_state(String::default);
    let description_handle = use_state(String::default);
    let error_message_handle = use_state(String::default);
    let loading_handle = use_state(|| false);
    let authors_handle = use_state(|| vec![]);

    let name = (*name_handle).clone();
    let code = (*code_handle).clone();
    let rustacean_id = (*rustacean_id_handle).clone();
    let version = (*version_handle).clone();
    let description = (*description_handle).clone();
    let error_message = (*error_message_handle).clone();
    let loading = *loading_handle;
    let authors = (*authors_handle).clone();

    // Determine if this is an edit operation
    let is_editing = props.crate_id.is_some();
    let button_text = if is_editing { "Update" } else { "Save" };

    // Fetch authors list and existing crate data if editing
    {
        let name_handle = name_handle.clone();
        let code_handle = code_handle.clone();
        let rustacean_id_handle = rustacean_id_handle.clone();
        let version_handle = version_handle.clone();
        let description_handle = description_handle.clone();
        let error_message_handle = error_message_handle.clone();
        let loading_handle = loading_handle.clone();
        let authors_handle = authors_handle.clone();
        let current_user_ctx = current_user_ctx.clone();
        let crate_id = props.crate_id;

        use_effect_with(
            (current_user_ctx.token.clone(), crate_id),
            move |(token, id)| {
                if let Some(token) = token {
                    loading_handle.set(true);
                    let token = token.clone();
                    let id = *id; // Dereference the Option<i32>
                    let name_handle = name_handle.clone();
                    let code_handle = code_handle.clone();
                    let rustacean_id_handle = rustacean_id_handle.clone();
                    let version_handle = version_handle.clone();
                    let description_handle = description_handle.clone();
                    let error_message_handle = error_message_handle.clone();
                    let loading_handle = loading_handle.clone();
                    let authors_handle = authors_handle.clone();

                    spawn_local(async move {
                        // First fetch authors
                        match api_rustaceans(&token).await {
                            Ok(rustaceans) => {
                                authors_handle.set(rustaceans.clone());

                                // Set default rustacean_id if not editing and not already set
                                if id.is_none() && rustacean_id_handle.len() == 0 {
                                    if let Some(first_author) = rustaceans.first() {
                                        rustacean_id_handle.set(first_author.id.to_string());
                                    }
                                }
                            }
                            Err(e) => {
                                error_message_handle.set(format!("Failed to load authors: {}", e));
                                loading_handle.set(false);
                                return;
                            }
                        }

                        // Then fetch existing crate data if editing
                        if let Some(id) = id {
                            match api_crate_show(&token, id).await {
                                Ok(a_crate) => {
                                    name_handle.set(a_crate.name);
                                    code_handle.set(a_crate.code);
                                    rustacean_id_handle.set(a_crate.rustacean_id.to_string());
                                    version_handle.set(a_crate.version);
                                    description_handle.set(a_crate.description.unwrap_or_default());
                                }
                                Err(e) => {
                                    error_message_handle.set(format!("Failed to load crate: {}", e))
                                }
                            }
                        }
                        loading_handle.set(false);
                    });
                }
            },
        );
    }

    let name_changed = create_input_callback(name_handle.clone());
    let code_changed = create_input_callback(code_handle.clone());
    let version_changed = create_input_callback(version_handle.clone());
    let description_changed = create_textarea_callback(description_handle.clone());
    let rustacean_id_changed = create_select_callback(rustacean_id_handle.clone());

    let onsubmit = create_submit_callback(
        name.clone(),
        code.clone(),
        rustacean_id.clone(),
        version.clone(),
        description.clone(),
        error_message_handle.clone(),
        navigator.clone(),
        current_user_ctx.clone(),
        props.crate_id,
    );

    if loading {
        return html! {
            <div class="text-center">
                <div class="spinner-border" role="status">
                    <span class="visually-hidden">{"Loading..."}</span>
                </div>
            </div>
        };
    }

    let options = authors
        .iter()
        .map(|r| {
            (
                AttrValue::from(r.id.to_string()),
                AttrValue::from(r.name.clone()),
            )
        })
        .collect::<Vec<(AttrValue, AttrValue)>>();

    html! {
        <form onsubmit={onsubmit}>
            if !error_message.is_empty() {
                <Alert alert_type={"danger"} message={error_message} />
            }
            <div class="mb-3">
                <Input
                    input_type="text"
                    name="code"
                    label="Code"
                    value={code}
                    onchange={code_changed}
                />
            </div>
            <div class="mb-3">
                <Input
                    input_type="text"
                    name="name"
                    label="Name"
                    value={name}
                    onchange={name_changed}
                />
            </div>
            <div class="mb-3">
                <Input
                    input_type="text"
                    name="version"
                    label="Version"
                    value={version}
                    onchange={version_changed}
                />
            </div>
            <div class="mb-3">
                <Select
                    name="author"
                    label="Author"
                    value={rustacean_id}
                    onchange={rustacean_id_changed}
                    options={options}
                />
            </div>
            <div class="mb-3">
                <Textarea
                    name="description"
                    label="Description"
                    value={description}
                    onchange={description_changed}
                />
            </div>
            <Button button_type="submit" class="btn btn-primary">
                {button_text}
            </Button>
        </form>
    }
}

fn create_input_callback(handle: UseStateHandle<String>) -> Callback<Event> {
    Callback::from(move |e: Event| {
        if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
            handle.set(input.value());
        }
    })
}

fn create_textarea_callback(handle: UseStateHandle<String>) -> Callback<Event> {
    Callback::from(move |e: Event| {
        if let Some(input) = e.target_dyn_into::<HtmlTextAreaElement>() {
            handle.set(input.value());
        }
    })
}

fn create_select_callback(handle: UseStateHandle<String>) -> Callback<Event> {
    Callback::from(move |e: Event| {
        if let Some(input) = e.target_dyn_into::<HtmlSelectElement>() {
            handle.set(input.value());
        }
    })
}

fn create_submit_callback(
    name: String,
    code: String,
    rustacean_id: String,
    version: String,
    description: String,
    error_handle: UseStateHandle<String>,
    navigator: Navigator,
    user_ctx: CurrentUserContext,
    crate_id: Option<i32>,
) -> Callback<SubmitEvent> {
    Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        handle_form_submission(
            name.clone(),
            code.clone(),
            rustacean_id.clone(),
            version.clone(),
            description.clone(),
            error_handle.clone(),
            navigator.clone(),
            user_ctx.clone(),
            crate_id,
        );
    })
}

fn handle_form_submission(
    name: String,
    code: String,
    rustacean_id: String,
    version: String,
    description: String,
    error_handle: UseStateHandle<String>,
    navigator: Navigator,
    user_ctx: CurrentUserContext,
    crate_id: Option<i32>,
) {
    match &user_ctx.token {
        Some(token) => {
            let Ok(parsed_rustacean_id) = rustacean_id.parse::<i32>() else {
                error_handle.set("Cannot parse rustacean ID".to_string());
                return;
            };

            let token = token.clone();
            spawn_local(async move {
                let result = match crate_id {
                    Some(id) => {
                        api_crate_update(
                            &token,
                            id,
                            name,
                            code,
                            parsed_rustacean_id,
                            version,
                            description,
                        )
                        .await
                    }
                    None => {
                        api_crate_create(
                            &token,
                            name,
                            code,
                            parsed_rustacean_id,
                            version,
                            description,
                        )
                        .await
                    }
                };

                match result {
                    Ok(_) => navigator.push(&Route::Crates),
                    Err(e) => error_handle.set(e.to_string()),
                }
            });
        }
        None => error_handle.set("Session expired. Please log in again".to_string()),
    }
}

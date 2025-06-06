use crate::api::user::{api_me, LoginResponse, MeResponse, User};
use gloo_storage::{SessionStorage, Storage};
use std::rc::Rc;
use yew::platform::spawn_local;
use ::yew::prelude::*;
use yew::{Reducible, UseReducerHandle};

const SESSION_TOKEN_KEY: &str = "cr8s_token";

pub type CurrentUserContext = UseReducerHandle<CurrentUser>;

#[derive(PartialEq)]
pub struct CurrentUser {
    pub user: Option<User>,
    pub token: Option<String>,
    pub is_loading: bool,
}

impl Default for CurrentUser {
    fn default() -> Self {
        // Start in loading state if there's a token to check
        let is_loading = SessionStorage::get::<String>(SESSION_TOKEN_KEY).is_ok();
        Self {
            user: None,
            token: None,
            is_loading,
        }
    }
}

pub enum CurrentUserAction {
    LoginSuccess { token: String, user: User },
    LoginFail,
}

impl Reducible for CurrentUser {
    type Action = CurrentUserAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            CurrentUserAction::LoginSuccess { token, user } => {
                if let Err(_) = SessionStorage::set(SESSION_TOKEN_KEY, token.clone()) {
                    // Handle storage error by treating as login failure
                    return Self {
                        user: None,
                        token: None,
                        is_loading: false,
                    }
                    .into();
                }
                Self {
                    user: Some(user),
                    token: Some(token),
                    is_loading: false,
                }
                .into()
            }
            CurrentUserAction::LoginFail => {
                SessionStorage::clear();
                Self {
                    user: None,
                    token: None,
                    is_loading: false,
                }
                .into()
            }
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(CurrentUserProvider)]
pub fn current_user_provider(props: &Props) -> Html {
    let user = use_reducer(CurrentUser::default);
    let initial_check = use_state(|| false);

    if !*initial_check {
        initial_check.set(true);
        if let Ok(token) = SessionStorage::get::<String>(SESSION_TOKEN_KEY) {
            let cloned_user = user.clone();
            spawn_local(async move {
                match api_me(&token).await {
                    Ok(me_response) => {
                        let user = User {
                            id: me_response.id,
                            username: me_response.username,
                            created_at: me_response.created_at,
                        };
                        cloned_user.dispatch(CurrentUserAction::LoginSuccess { token, user });
                    }
                    Err(_) => {
                        SessionStorage::clear();
                        cloned_user.dispatch(CurrentUserAction::LoginFail);
                    }
                }
            });
        }
    }

    html! {
        <ContextProvider<CurrentUserContext> context={user}>
            {props.children.clone()}
        </ContextProvider<CurrentUserContext>>
    }
}

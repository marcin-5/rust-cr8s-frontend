use crate::api::user::{api_me, LoginResponse, MeResponse, User};
use gloo_storage::{SessionStorage, Storage};
use std::rc::Rc;
use yew::platform::spawn_local;
use ::yew::prelude::*;
use yew::{Reducible, UseReducerHandle};

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
        let is_loading = SessionStorage::get::<String>("cr8s_token").is_ok();
        Self {
            user: None,
            token: None,
            is_loading,
        }
    }
}

pub enum CurrentUserActions {
    LoginSuccess,
    LoginFail,
    StartLoading,
}

pub struct CurrentUserDispatchActions {
    pub action_type: CurrentUserActions,
    pub login_response: Option<LoginResponse>,
    pub me_response: Option<MeResponse>,
}

impl Reducible for CurrentUser {
    type Action = CurrentUserDispatchActions;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action.action_type {
            CurrentUserActions::StartLoading => Self {
                user: self.user.clone(),
                token: self.token.clone(),
                is_loading: true,
            }
            .into(),
            CurrentUserActions::LoginSuccess => {
                let login_response = action.login_response.expect("Missing login response");
                let me_response = action.me_response.expect("Missing login response");
                SessionStorage::set("cr8s_token", login_response.token.clone()).unwrap();
                Self {
                    user: Some(User {
                        id: me_response.id,
                        username: me_response.username,
                        created_at: me_response.created_at,
                    }),
                    token: Some(login_response.token),
                    is_loading: false,
                }
                .into()
            }
            CurrentUserActions::LoginFail => {
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
        if let Ok(token) = SessionStorage::get::<String>("cr8s_token") {
            let cloned_user = user.clone();
            spawn_local(async move {
                match api_me(&token).await {
                    Ok(me_response) => {
                        cloned_user.dispatch(CurrentUserDispatchActions {
                            action_type: CurrentUserActions::LoginSuccess,
                            login_response: Some(LoginResponse { token }),
                            me_response: Some(me_response),
                        });
                    }
                    Err(_) => {
                        SessionStorage::clear();
                        cloned_user.dispatch(CurrentUserDispatchActions {
                            action_type: CurrentUserActions::LoginFail,
                            login_response: None,
                            me_response: None,
                        });
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

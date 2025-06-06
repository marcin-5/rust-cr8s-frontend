use crate::api::user::{LoginResponse, MeResponse, User};
use std::rc::Rc;
use ::yew::prelude::*;
use yew::{Reducible, UseReducerHandle};

pub type CurrentUserContext = UseReducerHandle<CurrentUser>;

#[derive(Default, PartialEq)]
pub struct CurrentUser {
    pub user: Option<User>,
    pub token: Option<String>,
}

pub enum CurrentUserActions {
    LoginSuccess,
    LoginFail,
}

pub struct CurrentUserDispatchActions {
    pub actions_type: CurrentUserActions,
    pub login_response: Option<LoginResponse>,
    pub me_response: Option<MeResponse>,
}

impl Reducible for CurrentUser {
    type Action = CurrentUserDispatchActions;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action.actions_type {
            CurrentUserActions::LoginSuccess => {
                let login_response = action.login_response.expect("Missing login response");
                let me_response = action.me_response.expect("Missing login response");
                Self {
                    user: Some(User {
                        id: me_response.id,
                        username: me_response.username,
                        created_at: me_response.created_at,
                    }),
                    token: Some(login_response.token),
                }
                .into()
            }
            CurrentUserActions::LoginFail => Self {
                user: None,
                token: None,
            }
            .into(),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(CurrentUserProvider)]
pub fn curent_user_provider(props: &Props) -> Html {
    let user = use_reducer(CurrentUser::default);

    html! {
        <ContextProvider<CurrentUserContext> context={user}>
            {props.children.clone()}
        </ContextProvider<CurrentUserContext>>
    }
}

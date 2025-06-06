use crate::api::API_URL;
use gloo_net::http::Request;
use gloo_net::Error;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct LoginResponse {
    pub token: String,
}

pub async fn api_login(username: String, password: String) -> Result<LoginResponse, Error> {
    let response = Request::post(&format!("{}/login", &*API_URL))
        .json(&json!({ "username": username, "password": password }))?
        .send()
        .await?;

    if !response.ok() {
        return Err(Error::GlooError(format!(
            "Login failed with status: {}",
            response.status()
        )));
    }

    response.json::<LoginResponse>().await
}

pub async fn api_me(token: &str) -> Result<User, Error> {
    let response = Request::get(&format!("{}/me", &*API_URL))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await?;

    if !response.ok() {
        return Err(Error::GlooError(format!(
            "Request failed with status: {}",
            response.status()
        )));
    }

    response.json::<User>().await
}

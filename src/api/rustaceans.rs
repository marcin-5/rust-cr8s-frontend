use crate::api::API_URL;
use gloo_net::http::Request;
use gloo_net::Error;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Clone)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

fn create_authenticated_url(endpoint: &str) -> String {
    format!("{}{}", &*API_URL, endpoint)
}

fn add_auth_header(token: &str) -> String {
    format!("Bearer {}", token)
}

pub async fn api_rustaceans(token: &str) -> Result<Vec<Rustacean>, Error> {
    let response = Request::get(&create_authenticated_url("/rustaceans"))
        .header("Authorization", &add_auth_header(token))
        .send()
        .await?;
    response.json::<Vec<Rustacean>>().await
}

pub async fn api_rustacean_create(
    token: &str,
    name: String,
    email: String,
) -> Result<Rustacean, Error> {
    let response = Request::post(&create_authenticated_url("/rustaceans"))
        .header("Authorization", &add_auth_header(token))
        .json(&json!({
            "name": name,
            "email": email
        }))?
        .send()
        .await?;
    response.json::<Rustacean>().await
}

use crate::api::API_URL;
use gloo_net::http::Request;
use gloo_net::Error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, PartialEq)]
pub struct Crate {
    pub id: i32,
    pub rustacean_id: i32,
    pub name: String,
    pub code: String,
    pub version: String,
    pub description: Option<String>,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct CrateData {
    pub name: String,
    pub code: String,
}

fn create_authenticated_url(endpoint: &str) -> String {
    format!("{}{}", &*API_URL, endpoint)
}

fn add_auth_header(token: &str) -> String {
    format!("Bearer {}", token)
}

pub async fn api_crates(token: &str) -> Result<Vec<Crate>, Error> {
    let response = Request::get(&create_authenticated_url("/crates"))
        .header("Authorization", &add_auth_header(token))
        .send()
        .await?;
    response.json::<Vec<Crate>>().await
}

pub async fn api_crate_create(token: &str, name: String, code: String) -> Result<Crate, Error> {
    let data = CrateData { name, code };
    let response = Request::post(&create_authenticated_url("/crates"))
        .header("Authorization", &add_auth_header(token))
        .json(&data)?
        .send()
        .await?;
    response.json::<Crate>().await
}

pub async fn api_crate_update(
    token: &str,
    id: i32,
    name: String,
    code: String,
) -> Result<Crate, Error> {
    let data = CrateData { name, code };
    let response = Request::put(&create_authenticated_url(&format!("/crates/{}", id)))
        .header("Authorization", &add_auth_header(token))
        .json(&data)?
        .send()
        .await?;
    response.json::<Crate>().await
}

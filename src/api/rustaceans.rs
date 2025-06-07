use crate::api::{add_auth_header, create_authenticated_url, API_URL};
use gloo_net::http::Request;
use gloo_net::Error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct RustaceanData {
    pub name: String,
    pub email: String,
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
    let data = RustaceanData { name, email };
    let response = Request::post(&create_authenticated_url("/rustaceans"))
        .header("Authorization", &add_auth_header(token))
        .json(&data)?
        .send()
        .await?;
    response.json::<Rustacean>().await
}

pub async fn api_rustacean_show(token: &str, id: i32) -> Result<Rustacean, Error> {
    let response = Request::get(&create_authenticated_url(&format!("/rustaceans/{}", id)))
        .header("Authorization", &add_auth_header(token))
        .send()
        .await?;
    response.json::<Rustacean>().await
}

pub async fn api_rustacean_update(
    token: &str,
    id: i32,
    name: String,
    email: String,
) -> Result<Rustacean, Error> {
    let data = RustaceanData { name, email };
    let response = Request::put(&create_authenticated_url(&format!("/rustaceans/{}", id)))
        .header("Authorization", &add_auth_header(token))
        .json(&data)?
        .send()
        .await?;
    response.json::<Rustacean>().await
}

pub async fn api_rustacean_delete(token: &String, id: i32) -> Result<(), Error> {
    let _ = Request::delete(&create_authenticated_url(&format!("/rustaceans/{}", id)))
        .header("Authorization", &add_auth_header(token))
        .send()
        .await?;
    Ok(())
}

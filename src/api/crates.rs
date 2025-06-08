use crate::api::{add_auth_header, create_authenticated_url};
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
    pub rustacean_id: i32,
    pub version: String,
    pub description: String,
}

pub async fn api_crate_show(token: &str, id: i32) -> Result<Crate, Error> {
    let response = Request::get(&create_authenticated_url(&format!("/crates/{}", id)))
        .header("Authorization", &add_auth_header(token))
        .send()
        .await?;
    response.json::<Crate>().await
}

pub async fn api_crates(token: &str) -> Result<Vec<Crate>, Error> {
    let response = Request::get(&create_authenticated_url("/crates"))
        .header("Authorization", &add_auth_header(token))
        .send()
        .await?;
    response.json::<Vec<Crate>>().await
}

pub async fn api_crate_create(
    token: &str,
    name: String,
    code: String,
    rustacean_id: i32,
    version: String,
    description: String,
) -> Result<Crate, Error> {
    let data = CrateData {
        name,
        code,
        rustacean_id,
        version,
        description,
    };
    let response = Request::post(&create_authenticated_url("/crates"))
        .header("Authorization", &add_auth_header(token))
        .header("Accept", "application/json")
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
    rustacean_id: i32,
    version: String,
    description: String,
) -> Result<Crate, Error> {
    let data = CrateData {
        name,
        code,
        rustacean_id,
        version,
        description,
    };
    let response = Request::put(&create_authenticated_url(&format!("/crates/{}", id)))
        .header("Authorization", &add_auth_header(token))
        .json(&data)?
        .send()
        .await?;
    response.json::<Crate>().await
}

pub async fn api_crate_delete(token: &str, id: i32) -> Result<(), Error> {
    let _ = Request::delete(&create_authenticated_url(&format!("/crates/{}", id)))
        .header("Authorization", &add_auth_header(token))
        .send()
        .await?;
    Ok(())
}

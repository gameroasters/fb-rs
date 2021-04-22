#![forbid(unsafe_code)]
#![deny(dead_code)]
#![deny(unused_must_use)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::panic)]
#![deny(clippy::perf)]
#![deny(clippy::nursery)]
#![deny(clippy::needless_update)]
#![deny(clippy::match_like_matches_macro)]
#![deny(clippy::from_over_into)]
#![deny(clippy::useless_conversion)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::upper_case_acronyms)]

mod error;
pub mod types;

use crate::{
    error::Result,
    types::{ResponseFriends, ResponsePicture, User},
};
use async_trait::async_trait;
use hyper::{body, Body, Client, Request};
use hyper_tls::HttpsConnector;

#[async_trait]
pub trait GraphAPI {
    async fn me(&self, token: &str) -> Result<User>;
    async fn friends(&self, token: &str) -> Result<ResponseFriends>;
    async fn my_picture(&self, token: &str) -> Result<ResponsePicture>;
}

#[derive(Default, Debug)]
pub struct FBGraphAPI {}

const BASE_URL: &str = "https://graph.facebook.com/v8.0";

#[async_trait]
impl GraphAPI for FBGraphAPI {
    async fn me(&self, token: &str) -> Result<User> {
        let url = format!(
            "{}/me?fields=id%2Cname%2Cfirst_name%2Clast_name%2Cemail&access_token={}",
            BASE_URL, token
        );

        let resp = get_request(&url).await?;
        let resp: User = serde_json::from_str(&resp)?;

        Ok(resp)
    }

    async fn friends(&self, token: &str) -> Result<ResponseFriends> {
        let url = format!(
            "{}/me/friends?fields=id%2Cname%2Cfirst_name%2Clast_name&access_token={}",
            BASE_URL, token
        );

        let resp = get_request(&url).await?;
        let resp: ResponseFriends = serde_json::from_str(&resp)?;

        Ok(resp)
    }

    async fn my_picture(&self, token: &str) -> Result<ResponsePicture> {
        let url = format!(
            "{}/me/picture?redirect=false&access_token={}",
            BASE_URL, token
        );

        let resp = get_request(&url).await?;
        let resp: ResponsePicture = serde_json::from_str(&resp)?;

        Ok(resp)
    }
}

async fn get_request(url: &str) -> Result<String> {
    let client = Client::builder().build::<_, Body>(HttpsConnector::new());
    let request = Request::builder()
        .method("GET")
        .uri(url)
        .body(Body::empty())?;
    let response = client.request(request).await?;

    let buf = body::to_bytes(response).await?;
    let res = String::from_utf8(buf.to_vec())?;

    Ok(res)
}

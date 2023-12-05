use reqwest;
use api_types::dto::LogFormDTO;

use crate::config::API_URL;

pub async fn logging(form: &LogFormDTO) -> Result<reqwest::Response, reqwest::Error> {
    reqwest::Client::new()
        .post(API_URL.to_string() + "/users/LogIn")
        .form(form)
        .send()
        .await
}
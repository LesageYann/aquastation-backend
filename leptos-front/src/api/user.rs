use leptos::logging::log;
use leptos::server_fn::serde::Deserialize;
use reqwest;

use api_types::container::{Container, ContainerWithStatus};
use api_types::dto::{CreateContainerFormDTO, CreateProbeFormDTO};
use crate::api::AQSRequest;
use crate::api::error::AQSError;

use crate::config::API_URL;


async fn extract_result<T>(req: Result<reqwest::Response, reqwest::Error>) -> AQSRequest<T>
    where
        T: for<'de> Deserialize<'de>
{
    match req {
        Ok(response) => match response.status() {
            reqwest::StatusCode::OK => {
                match response.json::<T>().await {
                    Ok(body) => {
                        Ok(body)
                    }
                    Err(_e) => {
                        Err(AQSError::InvalidData)
                    }
                }
            }
            _ => {
                // todo better error
                Err(AQSError::NetworkError)
            }
        },
        Err(_e) => {
            // todo better error
            Err(AQSError::NetworkError)
        }
    }
}


pub async fn fetch_user_container(bearer: String) -> Result<Vec<ContainerWithStatus>, AQSError> {
    log!("bearer ? {}", bearer);
    let req = reqwest::Client::new()
        .get(API_URL.to_string() + "/containers?probes_states=true")
        .header("Authorization", bearer.to_owned())
        .send()
        .await;

    extract_result::<Vec<ContainerWithStatus>>(req).await
}


pub async fn create_container(bearer: String, form: &CreateContainerFormDTO) -> Result<Container, AQSError> {
    log!("bearer ? {}", bearer);
    let req = reqwest::Client::new()
        .post(API_URL.to_string() + "/containers")
        .header("Authorization", bearer.to_owned())
        .form(form)
        .send()
        .await;

    extract_result::<Container>(req).await
}

pub async fn create_probe(bearer: String,container_id: i32 ,form: &CreateProbeFormDTO) -> Result<Container, AQSError> {
    log!("bearer ? {}", bearer);
    let req = reqwest::Client::new()
        .post(API_URL.to_string() + "/containers/" + &container_id.to_string() + "/probes")
        .header("Authorization", bearer.to_owned())
        .form(form)
        .send()
        .await;

    extract_result::<Container>(req).await
}

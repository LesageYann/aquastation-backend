use crate::container::ContainerType;

#[derive(serde::Deserialize)]
pub struct AuthResponse {
    pub auth: AuthDTO,
    pub user: UserDTO,
}

#[derive(serde::Deserialize)]
pub struct AuthDTO {
    pub access_token: String,
    pub token_type: String,
}

#[derive(serde::Deserialize)]
pub struct UserDTO {
    pub username: String,
    pub id: i32,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct LogFormDTO {
    pub password: String,
    pub email: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CreateContainerFormDTO {
    pub volume: i64,
    pub name: String,
    pub container_type: ContainerType,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CreateProbeFormDTO {
    pub min: i64,
    pub max: i64,
    pub name: String,
    pub probe_type: String,
}
use serde::{Deserialize};
use sqlx::PgPool;

use api_types::container::{ContainerType, Container};
use crate::data_sources::container::DataContainer;

use crate::usecases::users::claims::Claims;
use crate::model::error::ASError;

#[derive(Deserialize, Debug)]
pub struct CreateBody {
    pub name: String,
    pub volume: i64,
    pub container_type: ContainerType,
}

pub async fn create_container_usecase(pool: &PgPool, claims: Claims, payload: CreateBody) -> Result<Container, ASError> {
    let container = DataContainer::create(pool, payload.name, payload.container_type, payload.volume, claims.sub).await;
    match container {
        Some(cont) => { Ok(cont) }
        None => { Err(ASError::InternalError) }
    }
}

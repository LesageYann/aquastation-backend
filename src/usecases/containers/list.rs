use serde::Deserialize;
use sqlx::PgPool;

use crate::data_sources::container::DataContainer;
use crate::model::container::Container;

#[derive(Deserialize, Debug)]
pub struct ListContainerData {
    container_type: Option<String>,
    limit: Option<u64>,
    offset: Option<u64>,
    user_id: i32
}

pub async fn list_containers_usecase(pool: &PgPool, body: ListContainerData) -> Option<Vec<Container>> {
    DataContainer::list(pool, body.user_id).await
}

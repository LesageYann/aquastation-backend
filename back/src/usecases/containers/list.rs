use serde::Deserialize;
use serde_json::error::Category::Data;
use sqlx::PgPool;

use crate::data_sources::container::DataContainer;
use api_types::container::Container;
use crate::data_sources::probes::DataProbes;
use crate::usecases::probes;

#[derive(Deserialize, Debug)]
pub struct ListContainerData {
    pub container_type: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub user_id: i32
}

pub async fn list_containers_usecase(pool: &PgPool, body: ListContainerData) -> Option<Vec<Container>> {
    DataContainer::list(pool, body.user_id).await
}

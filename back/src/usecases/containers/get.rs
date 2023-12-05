use serde::Deserialize;
use sqlx::PgPool;

use crate::data_sources::container::DataContainer;
use api_types::container::Container;

#[derive(Deserialize, Debug)]
pub struct GetContainerData {
    pub user_id: i32,
    pub container_id: i32,
}

pub async fn get_container_usecase(pool: &PgPool, data: GetContainerData) -> Option<Container> {
    let query = DataContainer::get(pool, data.user_id, data.container_id).await;
    query
}

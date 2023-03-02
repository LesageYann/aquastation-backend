use crate::data_sources::probes::DataProbes;
use crate::model::probe::Probe;
use sqlx::PgPool;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct ListProbesData {
    limit: Option<u64>,
    offset: Option<u64>,
    container_id: i32,
}

pub async fn list_probes_usecase(pool: &PgPool, body: ListProbesData) -> Option<Vec<Probe>> {
    DataProbes::list(pool, body.container_id).await
}

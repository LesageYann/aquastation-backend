use api_types::probe::Probe;
use sqlx::PgPool;
use serde::{Deserialize};
use crate::data_sources::probes::DataProbes;

#[derive(Deserialize, Debug)]
pub struct GetProbeData {
    pub probe_id: i32,
    pub user_id: i32,
}

pub struct GetProbeResult {}

pub async fn get_probe_usecase(pool: &PgPool, data: GetProbeData) -> Option<Probe> {
    DataProbes::get(pool, data.probe_id).await
}
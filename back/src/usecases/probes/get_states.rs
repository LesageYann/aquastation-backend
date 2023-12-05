use std::collections::HashMap;
use serde::Deserialize;
use sqlx::PgPool;

use api_types::probes_status::ProbeState;

use crate::data_sources::probes::DataProbes;

#[derive(Deserialize, Debug)]
pub struct GetStatesData {
    pub containers_id: Vec<i32>,
    pub user_id: i32,
}

pub struct GetProbeResult {}

pub async fn get_states(pool: &PgPool, data: GetStatesData) -> Option<HashMap<i32,Vec<ProbeState>>> {
    DataProbes::get_states(pool, data.containers_id)
}
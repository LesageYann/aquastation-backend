use std::collections::HashMap;
use sqlx;
use sqlx::{PgPool, Result};

use api_types::probe::Probe;
use api_types::probes_status::{ProbeState, ProbeStatus};

pub struct DataProbes {}

impl DataProbes {
    pub async fn list(pool: &PgPool, container_id: i32) -> Option<Vec<Probe>> {
        let query = sqlx::query_as!(Probe,
        "SELECT * FROM core.probe WHERE container_id = $1",
        container_id
    );
        let result: Result<Vec<Probe>> = query
            .fetch_all(pool)
            .await;
        match result {
            Ok(value) => Some(value),
            Err(e) => {
                println!("{}", e);
                None
            }
        }
    }

    pub fn get_states(pool: &PgPool, containers_id: Vec<i32>)-> Option<HashMap<i32,Vec<ProbeState>>> {
        //TODO
        Some(containers_id.iter().fold(HashMap::new(), |mut acc, container| {
            acc.insert(container.clone(), vec![ProbeState {
                name: "example".to_string(),
                status: ProbeStatus::Perfect,
                probe_type: "PH".to_string(),
            }]);
            acc
        }))
    }

    pub async fn get(pool: &PgPool, probe_id: i32) -> Option<Probe> {
        let result: Result<Probe> = sqlx::query_as!(Probe,
        "SELECT * FROM core.probe WHERE id = $1",
        probe_id
    )
            .fetch_one(pool)
            .await;

        match result {
            Ok(value) => Some(value),
            Err(e) => {
                println!("{}", e);
                None
            }
        }
    }
}
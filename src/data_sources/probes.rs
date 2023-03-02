use sqlx;
use sqlx::{PgPool, Result};

use crate::model::probe::Probe;

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
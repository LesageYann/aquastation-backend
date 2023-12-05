use api_types::container::{ContainerType, Container};
use sqlx::postgres::PgPool;
use sqlx;
use api_types::probes_status::ProbeState;

pub struct DataContainer {}

impl DataContainer {
    pub async fn list(pool: &PgPool, user_id: i32) -> Option<Vec<Container>> {
        let result = sqlx::query_as!(Container,
        "SELECT id, name, container_type as \"container_type: ContainerType\", volume, red, green, blue, user_id
         FROM core.container WHERE user_id = $1",
        user_id
    )
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

    pub async fn get(pool: &PgPool, user_id: i32, container_id: i32) -> Option<Container> {
        let result = sqlx::query_as!(Container,
        "SELECT id, name, container_type as \"container_type: ContainerType\", volume, red, green, blue, user_id
        FROM core.container WHERE user_id = $1 AND id = $2",
            user_id,
            container_id
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

    pub async fn create(pool: &PgPool, name: String, container_types: ContainerType, volume: i64, user_id: i32) -> Option<Container> {
        let result = sqlx::query_as!(Container,
            "INSERT INTO core.container (name, container_type, volume, red, green, blue, user_id)
                VALUES ($1, $2, $3, 0, 0, 0, $4) RETURNING id, name, container_type as \"container_type: ContainerType\", volume, red, green, blue, user_id",
            name,
            container_types as _,
            volume,
            user_id
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
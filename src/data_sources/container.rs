use crate::model::container::{ContainerType, Container};
use sqlx::postgres::PgPool;
use sqlx;

pub struct DataContainer {}

impl DataContainer {

    pub async fn list(pool: & PgPool, user_id: i32) -> Option<Vec<Container>> {
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

    pub async fn get(pool: & PgPool, user_id: i32) -> Option<Container> {
        let result = sqlx::query_as!(Container,
        "SELECT id, name, container_type as \"container_type: ContainerType\", volume, red, green, blue, user_id
        FROM core.container WHERE id = $1",
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
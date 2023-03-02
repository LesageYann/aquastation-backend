use serde::{Serialize};

#[derive(sqlx::Type)]
#[sqlx(type_name = "container_type_enum")]
#[derive(Serialize,Debug)]
pub enum ContainerType {
    Fish,
    Vegetable,
    Other,
}

#[derive(Serialize, Debug)]
pub struct Container {
    pub id: i32,
    pub name: String,
    pub container_type: ContainerType,
    pub volume: i64,
    pub red: i32,
    pub green: i32,
    pub blue: i32,
    pub user_id: i32,
}

impl Container {
    pub fn new(id: i32,
               name: String,
               container_type: ContainerType,
               volume: i64,
               red: i32,
               green: i32,
               blue: i32,
               user_id: i32) -> Container {
        Container { id, name, container_type, volume, red, green, blue, user_id }
    }
}
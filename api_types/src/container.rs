use std::fmt;
use serde::{Deserialize, Serialize};

#[cfg(feature = "iterable")]
use enum_iterator::Sequence;

#[cfg(feature = "sqlx")]
use sqlx;
use crate::probes_status::{ProbeState};

#[derive(Serialize,Deserialize,Debug)]
#[cfg_attr(feature = "iterable", derive(Clone, Eq, PartialEq, Sequence))]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "container_type_enum"))]
pub enum ContainerType {
    Fish,
    Vegetable,
    Other,
}

impl fmt::Display for ContainerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContainerType::Fish => write!(f, "Fish"),
            ContainerType::Vegetable => write!(f, "Vegetable"),
            ContainerType::Other => write!(f, "Other!"),
        }
    }
}

#[derive(Serialize, Debug)]
#[cfg_attr(feature = "iterable", derive(Clone, Deserialize, Eq, PartialEq))]
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
        Container { id, name, container_type, volume, red, green, blue, user_id}
    }
}

#[derive(Serialize, Debug)]
#[cfg_attr(feature = "iterable", derive(Clone, Deserialize, Eq, PartialEq))]
pub struct ContainerWithStatus {
    pub status: Vec<ProbeState>,
    pub props: Container
}
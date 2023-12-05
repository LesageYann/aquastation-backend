use serde::{Serialize};
#[derive(Serialize, Debug)]
pub struct Probe {
    pub id: i32,
    pub name: String,
    pub unit: String,
    pub probe_type: String,
    pub min: f64,
    pub max: f64,
    pub container_id: i32,
}

impl Probe {
    pub fn new(id: i32,
               name: String,
               unit: String,
               probe_type: String,
               min: f64,
               max: f64,
               container_id: i32, ) -> Probe {
        Probe { id, name, unit, probe_type, min, max, container_id }
    }
}
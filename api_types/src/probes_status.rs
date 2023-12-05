use serde::{Serialize, Deserialize};

#[derive(Serialize, Clone, Debug)]
#[cfg_attr(feature = "iterable", derive(Deserialize, Eq, PartialEq))]
pub enum ProbeStatus {
    Perfect,
    Ok,
    Alert,
    Error
}

#[derive(Serialize, Clone, Debug)]
#[cfg_attr(feature = "iterable", derive(Deserialize, Eq, PartialEq))]
pub struct ProbeState {
    pub name: String,
    pub status: ProbeStatus,
    pub probe_type: String,
}
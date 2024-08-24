use std::net::IpAddr;
use mongodb::bson::Bson;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Host {
    pub id: String,
    pub gpus: i16,
    pub model: String,
    pub make: String,
    pub state: HostState,
    pub ip: IpAddr,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum HostState {
    Available, Committing(String), Working(String)
}

impl From<HostState> for Bson {
    fn from(value: HostState) -> Self {
        match value {
            HostState::Available => Bson::Null,
            HostState::Committing(job_id) => Bson::String(String::from("Committing:") + job_id.as_str()),
            HostState::Working(job_id) => Bson::String(String::from("Working:") + job_id.as_str()),
        }
    }
}

// impl From<Bson> for HostState {
//     fn from(value: Bson) -> Self {
//      WILL PROBABLY NEED THIS AT SOME POINT
//     }
// }
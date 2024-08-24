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
    pub ip: IpAddr
}

#[derive(Serialize, Deserialize, Debug)]
pub enum HostState {
    Available, Committing, Working
}

impl From<HostState> for Bson {
    fn from(value: HostState) -> Self {
        match value {
            HostState::Available => Bson::Int32(0),
            HostState::Committing => Bson::Int32(1),
            HostState::Working => Bson::Int32(2),
        }
    }
}
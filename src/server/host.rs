use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Host {
    pub id: String,
    pub gpus: i16,
    pub model: String,
    pub make: String,
    pub available: bool
}
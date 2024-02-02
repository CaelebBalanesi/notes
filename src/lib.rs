use serde::{Serialize, Deserialize};
use uuid::Uuid;

pub mod database;

pub struct Note {
    pub uuid: Uuid,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewNote {
    pub text: String,
}
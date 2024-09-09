use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    level: i32,
}

impl Player {
    pub fn new() -> Self {
        Self { id: None, level: 0 }
    }

    pub fn get_id(&self) -> &Option<ObjectId> {
        &self.id
    }

    pub fn get_level(&self) -> i32 {
        self.level
    }
}

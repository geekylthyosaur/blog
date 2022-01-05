use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Score {
    pub id: i32,
    pub owner_id: i32,
    pub score: i32,
}

#[derive(Deserialize)]
pub struct ScoreToCreate {
    pub owner_id: i32,
}

impl Score {
    pub fn to_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}

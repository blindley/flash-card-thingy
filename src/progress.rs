use serde::{Serialize, Deserialize};

const MAX_STREAK_HISTORY: usize = 8;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardProgress {
    pub card_uuid: uuid::Uuid,

    pub card_instance: i32,
    pub due: chrono::naive::NaiveDate,
    pub interval: i32,
    pub streak: u8,
    pub streak_history: Vec<u8>,
}

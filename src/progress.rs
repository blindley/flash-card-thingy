use serde::{Serialize, Deserialize};

const MAX_STREAK_HISTORY: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CardId {
    pub uuid: uuid::Uuid,
    pub instance: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardProgress {
    pub due: chrono::naive::NaiveDate,
    pub interval: i32,
    pub streak: u8,
    pub streak_history: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProgress {
    pub cards: std::collections::BTreeMap<CardId, CardProgress>,
}

impl UserProgress {
    pub fn new() -> UserProgress {
        use std::collections::BTreeMap;
        UserProgress { cards: BTreeMap::new() }
    }

    pub fn load<P: AsRef<std::path::Path>>(path: P)
        -> Result<UserProgress, Box<dyn std::error::Error>>
    {
        let json_str =
            std::fs::read_to_string(path)
            .unwrap_or_else(|_| "{}".into());

        let p = serde_json::from_str(&json_str)?;
        Ok(p)
    }
}

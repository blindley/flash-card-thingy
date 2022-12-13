use serde::{Serialize, Deserialize};

use chrono::naive::NaiveDate;
use std::collections::{VecDeque};

const MAX_STREAK_HISTORY: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CardId {
    pub uuid: uuid::Uuid,
    pub instance: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardProgress {
    pub due: NaiveDate,
    pub interval: i32,
    pub streak: u8,
    pub streak_history: VecDeque<u8>,
}

impl CardProgress {
    pub fn new() -> CardProgress
    {
        CardProgress {
            due: chrono::Local::now().date_naive(),
            interval: 0,
            streak: 0,
            streak_history: VecDeque::new(),
        }
    }

    pub fn pass(&mut self)
    {
        use chrono::naive::Days;
        self.interval = self.interval * 3 / 2 + 1;
        self.due = self.due + Days::new(self.interval as u64);
        self.streak += 1;
    }

    pub fn fail(&mut self)
    {
        self.due = chrono::Local::now().date_naive();
        self.interval = 0;
        self.streak_history.push_front(self.streak);
        self.streak = 0;
        if self.streak_history.len() > MAX_STREAK_HISTORY {
            self.streak_history.resize(MAX_STREAK_HISTORY, 0);
        }
    }
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
            .unwrap_or_else(|_| "{\"cards\":{}}".into());

        let p = serde_json::from_str(&json_str)?;
        Ok(p)
    }
}

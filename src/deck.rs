use serde::{Serialize, Deserialize};
use crate::card::Note;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck {
    pub cards: Vec<Note>,
}

impl Deck {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Deck, Box<dyn std::error::Error>>
    {
        let file_content = std::fs::read_to_string(&path)?;
        let mut deck: Deck = serde_json::from_str(&file_content)?;

        let mut rewrite = false;
        for card in deck.cards.iter_mut() {
            if card.get_field("uuid").is_none() {
                card.set_field("uuid", uuid::Uuid::new_v4().to_string());
                rewrite = true;
            }
        }

        if rewrite {
            deck.save(&path)?;
        }

        Ok(deck)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>>
    {
        std::fs::write(&path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }
}

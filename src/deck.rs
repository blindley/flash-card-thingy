use serde::{Serialize, Deserialize};
use crate::note::Note;
use std::path::Path;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DeckInner {
    pub notes: Vec<Note>,
}

#[derive(Debug, Clone)]
pub struct Deck(DeckInner);

impl Deck {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Deck, Box<dyn std::error::Error>>
    {
        let file_content = std::fs::read_to_string(&path)?;
        let mut deck: DeckInner = serde_json::from_str(&file_content)?;

        let mut rewrite = false;
        for card in deck.notes.iter_mut() {
            if card.uuid.is_nil() {
                card.uuid = Uuid::now_v1(b"jiefne");
                rewrite = true;
            }

            if card.template.is_empty() {
                card.template = "basic".into();
                rewrite = true;
            }
        }

        deck.notes.sort_by_key(|note| note.uuid);
        let deck = Deck(deck);

        if rewrite {
            deck.save(&path)?;
        }

        Ok(deck)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>>
    {
        std::fs::write(&path, serde_json::to_string_pretty(&self.0)?)?;
        Ok(())
    }

    pub fn notes(&self) -> &[Note]
    {
        return &self.0.notes
    }

    pub fn get_note(&self, uuid: Uuid) -> Option<&Note>
    {
        self.0.notes.binary_search_by_key(&uuid, |note| note.uuid)
            .ok()
            .and_then(|i| Some(&self.0.notes[i]))
    }
}

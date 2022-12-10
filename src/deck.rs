use serde::{Serialize, Deserialize};
use crate::card::Card;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck {
    pub cards: Vec<Card>,
}

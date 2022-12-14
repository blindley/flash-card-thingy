
use std::path::{PathBuf};
use std::collections::{VecDeque};
use crate::progress::{UserProgress, CardId};
use crate::deck::Deck;


pub struct SessionData {
    pub user_progress_filepath: std::path::PathBuf,
    pub user_progress: UserProgress,
    pub deck: Deck,
    pub due_cards: VecDeque<CardId>,
}

impl SessionData {
    pub fn new() -> SessionData
    {
        let user_progress_filepath = "data/users/sample-user/user-progress.json".into();

        let mut user_progress = {
            UserProgress::load(&user_progress_filepath).unwrap()
        };

        let deck = {
            let path = "data/decks/sample.json";
            Deck::load(path).unwrap()
        };

        let mut due_cards = VecDeque::new();

        for card in deck.notes().iter() {

            let uuid = card.uuid;
            let instances = card.instances;

            for instance in 1..=instances {
                let card_id = CardId { uuid, instance, };
                let card_progress = user_progress.add_card(card_id);
                if card_progress.due >= chrono::Local::now().date_naive() {
                    due_cards.push_back(card_id);
                }
            }
        }

        SessionData {
            user_progress_filepath, user_progress, deck, due_cards,
        }
    }

    pub fn next_card_html(&self) -> String {
        if self.due_cards.is_empty() {
            "<h1>No cards due</h1>".into()
        } else {
            let card_id = self.due_cards[0];
            match self.deck.get_note(card_id.uuid) {
                Some(note) => note.to_html(card_id.instance),
                None => "<h1>Error: card not found</h1>".into(),
            }
        }
    }

    pub fn pass(&mut self) {
        let card_id = self.due_cards[0];
        let card_progress = self.user_progress.cards.get_mut(&card_id).unwrap();
        card_progress.pass();
        self.due_cards.pop_front();
    }

    pub fn fail(&mut self) {
        let card_id = self.due_cards[0];
        let card_progress = self.user_progress.cards.get_mut(&card_id).unwrap();
        card_progress.fail();
        self.due_cards.push_back(card_id);
        self.due_cards.pop_front();
    }


}

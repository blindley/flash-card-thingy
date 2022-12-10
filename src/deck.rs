use crate::card::Card;

#[derive(Debug, Clone, PartialEq, Eq)]
struct PriorityCard(pub Card);

fn effective_priority(card: &Card) -> f32
{
    card.get_field("priority")
        .and_then(|v| v.parse::<f32>().ok())
        .and_then(|v| if v.is_nan() { Some(0.0) } else { Some(v) })
        .unwrap_or(0.0)
}

impl std::cmp::Ord for PriorityCard {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let p0 = effective_priority(&self.0);
        let p1 = effective_priority(&other.0);

        p0.partial_cmp(&p1).unwrap()
            .then_with(|| {
                self.0.to_javascript_object(None)
                    .cmp(&other.0.to_javascript_object(None))
            })
    }
}

impl std::cmp::PartialOrd for PriorityCard {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Deck {
    cards: Vec<PriorityCard>,
}

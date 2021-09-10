use crate::Deck;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn execute(mut deck: Deck) -> Deck {
    deck.cards.shuffle(&mut thread_rng());
    deck
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn shuffling() {
        let original = mock_deck();
        let shuffled = execute(mock_deck());
        assert_ne!(shuffled.cards, original.cards);
    }

    fn mock_deck() -> Deck {
        Deck::new()
    }
}

use crate::Card;
use rand::thread_rng;
use rand::seq::SliceRandom;

pub fn execute(mut cards: Vec<Card>) -> Vec<Card> {
    cards.shuffle(&mut thread_rng());
    cards
}

#[cfg(test)] 
mod test {

    use super::*;

    #[test]
    fn shuffling() {
        let original = mock_deck();
        let shuffled = execute(mock_deck());
        assert_ne!(shuffled, original);
    }

    fn mock_deck() -> Vec<Card> {
        vec![
            Card::new(
                String::from("1"),
                String::from("Star Platinum"),
                String::from("Jotaro"),
                String::from("some desc"),
                false,
                String::from("image"),
                vec![],
            ),
            Card::new(
                String::from("2"),
                String::from("Hierophant Green"),
                String::from("Kakyoin"),
                String::from("some desc"),
                false,
                String::from("image"),
                vec![],
            ),
            Card::new(
                String::from("3"),
                String::from("Magician Red"),
                String::from("Avdol"),
                String::from("some desc"),
                false,
                String::from("image"),
                vec![],
            )
        ]
    }

}

mod cards;
mod usecases;

use cards::models::Deck;
use cards::models::Card;
use cards::models::CardAttribute;
use cards::models::Attribute;

use usecases::compare_two_cards_usecase;
use usecases::shuffle_deck_usecase;

fn main() {
    
    testGame()
    
}

fn testGame() {

    let mut deck = Deck::new();
    deck = shuffle_deck_usecase::execute(deck);
    let (cards1, cards2)= deck.split();
    // println!("vec1DECK: {:?}", cards1);
    // println!("vec2DECK: {:?}", cards2);

    play(cards1, cards2)

}

fn play(mut deck1: Deck, mut deck2: Deck) {
    
    while deck1.isEmpty() == false && deck2.isEmpty() == false {
        println!("deck1 len = {:?}, deck2 len = {:?}", deck1.cards.len(), deck2.cards.len());
        let a = deck1.draw();
        let b = deck2.draw();

        let winner = compare_two_cards_usecase::execute(&a, &b, &Attribute::POWER);
        println!("Fight - POWER! {:?} vs {:?} -> {:?}", a.get_title(), b.get_title(), winner.unwrap().get_title());

        if winner.unwrap().get_title() == a.get_title() {
            deck1.add_card(a);
            deck1.add_card(b);
        } else {
            deck2.add_card(a);
            deck2.add_card(b);
        }
    }

    println!("deck1 len = {:?}, deck2 len = {:?}", deck1.cards.len(), deck2.cards.len());
}

fn card1() -> Card {
    Card::new(
        String::from("1"),
        String::from("Star Platinum"),
        String::from("Jotaro"),
        String::from("some desc"),
        false,
        String::from("image"),
        vec![
            CardAttribute::new(Attribute::POWER, 5),
            CardAttribute::new(Attribute::SPEED, 5),
            CardAttribute::new(Attribute::RANGE, 5),
            CardAttribute::new(Attribute::STAMINA, 5),
            CardAttribute::new(Attribute::PRECISION, 5),
            CardAttribute::new(Attribute::DEVELOPMENT, 5),
        ],
    )
}

fn card2() -> Card {
    Card::new(
        String::from("2"),
        String::from("Hierophant Green"),
        String::from("Kakyoin"),
        String::from("some desc"),
        false,
        String::from("image"),
        vec![
            CardAttribute::new(Attribute::POWER, 4),
            CardAttribute::new(Attribute::SPEED, 10),
            CardAttribute::new(Attribute::RANGE, 10),
            CardAttribute::new(Attribute::STAMINA, 4),
            CardAttribute::new(Attribute::PRECISION, 6),
            CardAttribute::new(Attribute::DEVELOPMENT, 10),
        ],
    )
}

fn card3() -> Card {
    Card::new(
        String::from("3"),
        String::from("Magician Red"),
        String::from("Avdol"),
        String::from("some desc"),
        false,
        String::from("image"),
        vec![
            CardAttribute::new(Attribute::POWER, 3),
            CardAttribute::new(Attribute::SPEED, 6),
            CardAttribute::new(Attribute::RANGE, 9),
            CardAttribute::new(Attribute::STAMINA, 8),
            CardAttribute::new(Attribute::PRECISION, 8),
            CardAttribute::new(Attribute::DEVELOPMENT, 4),
        ],
    )
}

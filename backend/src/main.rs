mod cards;
mod usecases;

use cards::models::Deck;
use cards::models::Card;
use cards::models::CardAttribute;
use cards::models::Attribute;

use usecases::compare_two_cards_usecase;
use usecases::shuffle_deck_usecase;

fn main() {
    // compare_two_cards_usecase::execute(&card1(), &card2(), &Attribute::DEVELOPMENT);
    // shuffle_deck_usecase::execute(vec![card1(),card2(),card3()]);
    let mut deck = Deck::new();
    println!("deck is {:?}", deck);
    deck.remove_card();
    deck.remove_card();
    deck.remove_card();
    println!(" ");
    println!(" ");
    println!(" ");
    println!("deck is {:?}", deck);
    deck.add_card(card1());
    println!(" ");
    println!(" ");
    println!(" ");
    println!("deck is {:?}", deck);
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
            CardAttribute::new(Attribute::POWER, 9),
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
            CardAttribute::new(Attribute::POWER, 5),
            CardAttribute::new(Attribute::SPEED, 6),
            CardAttribute::new(Attribute::RANGE, 9),
            CardAttribute::new(Attribute::STAMINA, 8),
            CardAttribute::new(Attribute::PRECISION, 8),
            CardAttribute::new(Attribute::DEVELOPMENT, 4),
        ],
    )
}

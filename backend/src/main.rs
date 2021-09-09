mod cards;
mod usecases;

use cards::models::Card;
use cards::models::CardAttribute;
use cards::models::Attribute;

use usecases::compare_two_cards_usecase;

fn main() {
    compare_two_cards_usecase::compare(&card1(), &card2(), &Attribute::DEVELOPMENT);
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
            CardAttribute::new(Attribute::POWER, 10),
            CardAttribute::new(Attribute::SPEED, 12),
            CardAttribute::new(Attribute::RANGE, 1),
            CardAttribute::new(Attribute::STAMINA, 5),
            CardAttribute::new(Attribute::PRECISION, 8),
            CardAttribute::new(Attribute::DEVELOPMENT, 6),
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

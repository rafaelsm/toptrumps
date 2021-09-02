mod cards;

use cards::models::Card;
use cards::models::CardAttribute;
use cards::models::Attribute;

fn main() {
    let card = Card::new(
        String::from("asdad"),
        String::from("asdad"),
        String::from("asdad"),
        false,
        String::from("asdad"),
        vec![
            CardAttribute::new(Attribute::POWER, 10),
            CardAttribute::new(Attribute::SPEED, 12),
            CardAttribute::new(Attribute::RANGE, 1),
            CardAttribute::new(Attribute::STAMINA, 5),
            CardAttribute::new(Attribute::PRECISION, 8),
            CardAttribute::new(Attribute::DEVELOPMENT, 6),
        ],
    );
    println!("Hello, attribute {:?}", card);
}

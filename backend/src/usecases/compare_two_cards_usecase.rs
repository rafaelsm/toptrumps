use crate::Card;
use crate::Attribute;
use crate::CardAttribute;

pub fn compare<'a>(
    first_card: &'a Card, 
    second_card: &'a Card,
    attribute: &Attribute,
) -> Option<&'a Card> {

    println!("attr to compare {:?}", attribute);
    
    let a1 = find_attribute(first_card, attribute);
    let a2 = find_attribute(second_card, attribute);

    let a1val = a1.unwrap();
    let a2val = a2.unwrap();

    if a1val.get_value() > a2val.get_value() {
        return Some(first_card)
    } else if a1val.get_value() < a2val.get_value() {
        return Some(second_card)
    } else {
        return None
    };

}

fn find_attribute<'a>(card: &'a Card, attribute: &Attribute) -> Option<&'a CardAttribute> {
    card.get_attributes().iter().find(
        |card_attr| {
            card_attr.get_attribute() == attribute
        }
    )
}

#[cfg(test)]
mod test {

    use super::*;


    #[test]
    fn attribute_comparison_first_card_wins() {
        let first_card = card1(); 
        let second_card = card2(); 
        let winner = compare(&first_card, &second_card, &Attribute::SPEED);
        assert_eq!(&first_card, winner.unwrap())
    }

    #[test]
    fn attribute_comparison_second_card_wins() {
        let first_card = card1(); 
        let second_card = card2(); 
        let winner = compare(&first_card, &second_card, &Attribute::DEVELOPMENT);
        assert_eq!(&second_card, winner.unwrap())
    }


    #[test]
    fn attribute_comparison_draw() {
        let first_card = card1();
        let second_card = card2();
        let winner = compare(&first_card, &second_card, &Attribute::STAMINA);
        assert_eq!(None, winner)
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
                CardAttribute::new(Attribute::STAMINA, 5),
                CardAttribute::new(Attribute::PRECISION, 6),
                CardAttribute::new(Attribute::DEVELOPMENT, 10),
            ],
        )
    }

}
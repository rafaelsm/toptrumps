use rand::{Rng, prelude::IteratorRandom};

#[derive(Debug, PartialEq, Clone)]
pub enum Attribute {
    POWER,
    SPEED,
    RANGE,
    STAMINA,
    PRECISION,
    DEVELOPMENT,
}

impl Attribute {

    pub fn random_attribute() -> Attribute {
        // Self::all()
        //     .choose(2)
        Attribute::POWER
    }

}

#[derive(Debug, PartialEq, Clone)]
pub struct Card {
    id: String,
    title: String,
    subtitle: String,
    description: String,
    toptrump: bool,
    image_url: String,
    attributes: Vec<CardAttribute>,
}

impl Card {

    pub fn new(
        id: String,
        title: String,
        subtitle: String,
        description: String,
        toptrump: bool,
        image_url: String,
        attributes: Vec<CardAttribute>) -> Card {

        Card {
            id: id,
            title: title,
            subtitle: subtitle,
            description: description,
            toptrump: toptrump,
            image_url: image_url,
            attributes: attributes,
        }

    }

    pub fn get_attributes(&self) -> &Vec<CardAttribute> {
        &self.attributes
    } 

    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

}

#[derive(Debug, PartialEq, Clone)]
pub struct CardAttribute {
    attribute: Attribute,
    value: i32,
}

impl CardAttribute {

    pub fn new(
        attribute: Attribute, 
        value: i32,
    ) -> CardAttribute {

        CardAttribute {
            attribute: attribute,
            value: value,
        }

    }

    pub fn get_attribute(&self) -> &Attribute {
        &self.attribute
    }

    pub fn get_value(&self) -> &i32 {
        &self.value
    }

}

#[derive(Debug)]
pub struct Deck{
    pub cards: Vec<Card>
}

impl Deck {

    pub fn add_card(&mut self, card: Card) {
        self.cards.insert(self.cards.len() - 1, card)
    }

    pub fn remove_card(&mut self) {
        self.cards.drain(0..1);
    }

    pub fn split(&mut self) -> (Deck, Deck) {
        let (cards1, cards2) = self.cards.split_at(self.cards.len() / 2);
        let half = Deck { cards: cards1.to_vec() };
        let other_half = Deck { cards: cards2.to_vec() };
        (half, other_half)
    }

    pub fn is_empty(&self) -> bool {
        self.cards.len() <= 0
    }

    pub fn draw(&mut self) -> Card {
        self.cards.pop().unwrap()
    }

    pub fn next_card(&self) -> &Card {
        self.cards.first().unwrap()
    }

    pub fn new_with(cards: Vec<Card>) -> Deck {
        Deck {
            cards: cards
        }
    }

}
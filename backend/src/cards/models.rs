#[derive(Debug, PartialEq, Clone)]
pub enum Attribute {
    POWER,
    SPEED,
    RANGE,
    STAMINA,
    PRECISION,
    DEVELOPMENT,
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
pub struct Deck {
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
        let otherHalf = Deck { cards: cards2.to_vec() };
        (half, otherHalf)
    }

    pub fn isEmpty(&self) -> bool {
        self.cards.len() <= 0
    }

    pub fn  draw(&mut self) -> Card {
        self.cards.pop().unwrap()
    }

    fn newWith(cards: Vec<Card>) -> Deck {
        Deck {
            cards: cards
        }
    }

    pub fn new() -> Deck {
        Deck {
            cards : vec![
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
                ),
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
                ),
                Card::new(
                    String::from("3"),
                    String::from("Magician Red"),
                    String::from("Avdol"),
                    String::from("some desc"),
                    false,
                    String::from("image"),
                    vec![
                        CardAttribute::new(Attribute::POWER, 4),
                        CardAttribute::new(Attribute::SPEED, 6),
                        CardAttribute::new(Attribute::RANGE, 9),
                        CardAttribute::new(Attribute::STAMINA, 8),
                        CardAttribute::new(Attribute::PRECISION, 8),
                        CardAttribute::new(Attribute::DEVELOPMENT, 4),
                    ],
                ),
                Card::new(
                    String::from("4"),
                    String::from("Eremit purple"),
                    String::from("Joseph"),
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
                ),
                Card::new(
                    String::from("5"),
                    String::from("Silver chariot"),
                    String::from("Polnareff"),
                    String::from("some desc"),
                    false,
                    String::from("image"),
                    vec![
                        CardAttribute::new(Attribute::POWER, 2),
                        CardAttribute::new(Attribute::SPEED, 6),
                        CardAttribute::new(Attribute::RANGE, 9),
                        CardAttribute::new(Attribute::STAMINA, 8),
                        CardAttribute::new(Attribute::PRECISION, 8),
                        CardAttribute::new(Attribute::DEVELOPMENT, 4),
                    ],
                ),
                Card::new(
                    String::from("6"),
                    String::from("The World"),
                    String::from("DIO"),
                    String::from("some desc"),
                    false,
                    String::from("image"),
                    vec![
                        CardAttribute::new(Attribute::POWER, 10),
                        CardAttribute::new(Attribute::SPEED, 10),
                        CardAttribute::new(Attribute::RANGE, 10),
                        CardAttribute::new(Attribute::STAMINA, 10),
                        CardAttribute::new(Attribute::PRECISION, 8),
                        CardAttribute::new(Attribute::DEVELOPMENT, 9),
                    ],
                ),
                Card::new(
                    String::from("7"),
                    String::from("Death 13"),
                    String::from("Baby"),
                    String::from("some desc"),
                    false,
                    String::from("image"),
                    vec![
                        CardAttribute::new(Attribute::POWER, 6),
                        CardAttribute::new(Attribute::SPEED, 8),
                        CardAttribute::new(Attribute::RANGE, 9),
                        CardAttribute::new(Attribute::STAMINA, 7),
                        CardAttribute::new(Attribute::PRECISION, 8),
                        CardAttribute::new(Attribute::DEVELOPMENT, 4),
                    ],
                ),
                Card::new(
                    String::from("8"),
                    String::from("Justice"),
                    String::from("Enya"),
                    String::from("some desc"),
                    false,
                    String::from("image"),
                    vec![
                        CardAttribute::new(Attribute::POWER, 7),
                        CardAttribute::new(Attribute::SPEED, 8),
                        CardAttribute::new(Attribute::RANGE, 9),
                        CardAttribute::new(Attribute::STAMINA, 7),
                        CardAttribute::new(Attribute::PRECISION, 8),
                        CardAttribute::new(Attribute::DEVELOPMENT, 4),
                    ],
                ),
                Card::new(
                    String::from("9"),
                    String::from("Emperor"),
                    String::from("Hol Horse"),
                    String::from("some desc"),
                    false,
                    String::from("image"),
                    vec![
                        CardAttribute::new(Attribute::POWER, 8),
                        CardAttribute::new(Attribute::SPEED, 8),
                        CardAttribute::new(Attribute::RANGE, 9),
                        CardAttribute::new(Attribute::STAMINA, 7),
                        CardAttribute::new(Attribute::PRECISION, 8),
                        CardAttribute::new(Attribute::DEVELOPMENT, 4),
                    ],
                ),
                Card::new(
                    String::from("10"),
                    String::from("The Fool"),
                    String::from("Iggy"),
                    String::from("some desc"),
                    false,
                    String::from("image"),
                    vec![
                        CardAttribute::new(Attribute::POWER, 0),
                        CardAttribute::new(Attribute::SPEED, 6),
                        CardAttribute::new(Attribute::RANGE, 4),
                        CardAttribute::new(Attribute::STAMINA, 10),
                        CardAttribute::new(Attribute::PRECISION, 9),
                        CardAttribute::new(Attribute::DEVELOPMENT, 9),
                    ],
                ),
            ]
        }
    }
}
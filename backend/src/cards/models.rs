#[derive(Debug)]
pub enum Attribute {
    POWER,
    SPEED,
    RANGE,
    STAMINA,
    PRECISION,
    DEVELOPMENT,
}

#[derive(Debug)]
pub struct Card {
    id: String,
    uuid: String,
    name: String,
    toptrump: bool,
    image_url: String,
    attributes: Vec<CardAttribute>,
}

impl Card {

    pub fn new(
        id: String,
        uuid: String,
        name: String,
        toptrump: bool,
        image_url: String,
        attributes: Vec<CardAttribute>) -> Card {

        Card {
            id: id,
            uuid: uuid,
            name: name,
            toptrump: toptrump,
            image_url: image_url,
            attributes: attributes,
        }

    }

}

#[derive(Debug)]
pub struct CardAttribute {
    attribute: Attribute,
    value: i32,
}

impl CardAttribute {

    pub fn new(
        attribute: Attribute, 
        value: i32
    ) -> CardAttribute {

        CardAttribute {
            attribute: attribute,
            value: value,
        }

    }

}

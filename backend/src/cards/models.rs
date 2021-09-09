#[derive(Debug, PartialEq)]
pub enum Attribute {
    POWER,
    SPEED,
    RANGE,
    STAMINA,
    PRECISION,
    DEVELOPMENT,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

    pub fn get_attribute(&self) -> &Attribute {
        &self.attribute
    }

    pub fn get_value(&self) -> &i32 {
        &self.value
    }

}

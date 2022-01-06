use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Stand {
    id: u8,
    name: String,
}

impl Stand {
    pub fn new(id: u8, name: String) -> Stand {
        Self { id, name }
    }
}

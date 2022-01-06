use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GameRoom {
    pub name: String,
    pub password: String,
}

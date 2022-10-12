use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Chore {
    pub name: String,
    pub due: String,
    pub owner: Owner,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub enum Owner {
    Linus,
    Johannes,
}

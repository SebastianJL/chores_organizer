#[derive(serde::Deserialize, serde::Serialize)]
pub struct Chore {
    pub name: String,
    pub due: String,
}
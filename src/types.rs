#[derive(Clone, serde::Deserialize, serde::Serialize, Debug)]
pub struct Category {
    pub name: String,
    pub value: String,
}
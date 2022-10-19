fn default_string() -> String {
    "-".to_string()
}

#[derive(Clone, serde::Deserialize, serde::Serialize, Debug)]
pub struct Category {
    pub name: String,
    pub value: String,
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct Artifact {
    pub name: String,
    pub max_rarity: u8,
    #[serde(rename = "2-piece_bonus", default = "default_string")]
    pub two_piece_bonus: String,
    #[serde(rename = "4-piece_bonus", default = "default_string")]
    pub four_piece_bonus: String,
}
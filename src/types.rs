use crate::theme::Icon;

fn default_string() -> String {
    "".to_string()
}

fn default_usize() -> u8 {
    0
}

fn default_vec() -> Vec<String> {
    vec![]
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Category {
    pub name: String,
    pub value: String,
}

impl Category {
    pub fn icon(self) -> String {
        match self.value.as_str() {
            "artifacts" => Icon::ARTIFACT.icon,
            "boss" => Icon::CIRCLE.icon,
            "characters" => Icon::AMBER.icon,
            "consumables" => Icon::ITEM.icon,
            "domains" => Icon::CASTLE.icon,
            "elements" => Icon::ANEMO.icon,
            "enemies" => Icon::MONSTER.icon,
            "materials" => Icon::MATERIAL.icon,
            "nations" => Icon::COMPASS.icon,
            "weapons" => Icon::WEAPON.icon,
            _ => Icon::AMBER.icon,
        }.to_string()
    }
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct ArtifactSet {
    pub name: String,
    pub max_rarity: u8,
    #[serde(rename = "2-piece_bonus", default = "default_string")]
    pub two_piece_bonus: String,
    #[serde(rename = "4-piece_bonus", default = "default_string")]
    pub four_piece_bonus: String,
    #[serde(skip, default="default_vec")]
    pub icon: Vec<String>,
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct Artifact {
    pub name: String,
    pub rarity: String,
    pub set: String,
    #[serde(skip, default="default_string")]
    pub icon: String,
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct Character {
    pub affiliation: String,
    pub birthday: String,
    pub constellation: String,
    pub constellations: Vec<Talent>,
    pub description: String,
    pub name: String,
    pub nation: String,
    #[serde(rename = "passiveTalents")]
    pub passive_talents: Vec<Talent>,
    pub rarity: u8,
    #[serde(rename = "skillTalents")]
    pub skill_talents: Vec<Talent>,
    pub title: String,
    pub vision: String,
    pub vision_key: String,
    pub weapon: String,
    pub weapon_type: String,
    #[serde(skip, default="default_string")]
    pub icon: String,
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct Talent {
    pub description: String,
    #[serde(default = "default_usize")]
    pub level: u8,
    pub name: String,
    pub unlock: String,
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct Food {
    pub description: String,
    pub effect: String,
    #[serde(rename = "hasRecipe")]
    pub has_recipe: bool,
    pub name: String,
    #[serde(default = "default_usize")]
    pub proficiency: u8,
    pub rarity: u8,
    #[serde(default = "default_recipe")]
    pub recipe: Vec<Recipe>,
    #[serde(rename = "type")]
    pub food_type: String,
    #[serde(skip, default="default_string")]
    pub icon: String,
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct Recipe {
    pub item: String,
    pub quantity: u8,
}

fn default_recipe() -> Vec<Recipe> {
    vec![]
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct Potion {
    pub effect: String,
    pub name: String,
    pub rarity: u8,
    pub crafting: Vec<Recipe>,
    #[serde(skip, default="default_string")]
    pub icon: String,
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct Domain {
    pub description: String,
    pub location: String,
    pub name: String,
    pub nation: String,
    #[serde(rename = "recommendedElements")]
    pub recommended_elements: Vec<String>,
    pub requirements: Vec<Requirement>,
    pub rewards: Vec<Reward>,
    #[serde(rename = "type")]
    pub domain_type: String,
    #[serde(skip, default="default_string")]
    pub icon: String,
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct Requirement {
    #[serde(rename = "adventureRank")]
    pub adventure_rank: u8,
    pub level: u8,
    #[serde(rename = "leyLineDisorder")]
    pub ley_line_disorder: Vec<String>,
    #[serde(rename = "recommendedLevel")]
    pub recommended_level: u8
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct Reward {
    pub day: String,
    pub details: Vec<RewardDetail>,
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct RewardDetail {
    #[serde(rename = "adventureExperience")]
    pub adventure_experience: u8,
    #[serde(rename = "companionshipExperience")]
    pub companionship_experience: u8,
    #[serde(default = "default_drop")]
    pub drops: Vec<Drop>,
    #[serde(default = "default_drop")]
    pub items: Vec<Drop>,
    pub level: u8,
    pub mora: u16,
}

fn default_drop() -> Vec<Drop> {
    vec![]
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct Drop {
    pub drop_max: u8,
    pub drop_min: u8,
    pub name: String,
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct Element {
    pub key: String,
    pub name: String,
    pub reactions: Vec<Reaction>,
    #[serde(skip, default="default_string")]
    pub icon: String,
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct Reaction {
    pub description: String,
    pub name: String,
    pub elements: Vec<String>,
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct Enemy {
    #[serde(default="default_artifact")]
    pub artifacts: Vec<Artifact>,
    pub description: String,
    #[serde(default="default_enemy_drop")]
    pub drops: Vec<EnemyDrop>,
    #[serde(rename = "elemental-description", default="default_elemental_description")]
    pub elemental_description: Vec<ElementalDescription>,
    pub elements: Vec<String>,
    #[serde(default="default_string")]
    pub faction: String,
    pub family: String,
    pub id: String,
    #[serde(rename = "mora-gained", default="default_usize")]
    pub mora_gained: u8,
    pub name: String,
    pub region: String,
    #[serde(rename = "type")]
    pub enemy_type: String,
    #[serde(skip, default="default_string")]
    pub icon: String,
}

fn default_artifact() -> Vec<Artifact> {
    vec![]
}
fn default_enemy_drop() -> Vec<EnemyDrop> {
    vec![]
}
fn default_elemental_description() -> Vec<ElementalDescription> {
    vec![]
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct EnemyDrop {
    #[serde(rename = "minimum-level")]
    pub minimum_level: u8,
    pub rarity: u8,
    pub name: String,
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct ElementalDescription {
    pub description: String,
    pub element: String,
}
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Pokemon {
  pub id: i32,
  pub name: String,
  pub base_experience: i32,
  pub height: i8,
  pub is_default: bool,
  pub order: i32,
  pub weight: i16,
  pub forms: Vec<Form>,
  pub held_items: Vec<HeldItem>,
  pub moves: Vec<MoveGroup>,
  pub stats: Vec<StatGroup>,
  pub types: Vec<TypeGroup>,
  pub location_area_encounters: String,
  pub abilities: Vec<Abilities>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Form {
  url: String,
  name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Abilities {
  pub is_hidden: bool,
  pub slot: i8,
  pub ability: Ability,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ability {
  pub url: String,
  pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TypeGroup {
  slot: i8,
  r#type: Type,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Type {
  name: String,
  url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatGroup {
  base_stat: i16,
  effort: i8,
  stat: Stat,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stat {
  name: String,
  url: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct HeldItem {
  item: Item,
  version_details: Vec<VersionDetail>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
  name: String,
  url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VersionDetail {
  rarity: i8,
  version: Version,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
  name: String,
  url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MoveGroup {
  r#move: Move,
  version_group_details: Vec<VersionGroupDetail>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Move {
  name: String,
  url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VersionGroupDetail {
  level_learned_at: i8,
  move_learn_method: MoveLearnMethod,
  version_group: VersionGroup,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MoveLearnMethod {
  name: String,
  url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VersionGroup {
  name: String,
  url: String,
}

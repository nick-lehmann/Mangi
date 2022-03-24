use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Diet {
    Omnivore,
    Vegetarian,
    Vegan,
}

impl TryFrom<String> for Diet {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "omnivore" => Ok(Diet::Omnivore),
            "vegetarian" => Ok(Diet::Vegetarian),
            "vegan" => Ok(Diet::Vegan),
            _ => Err(format!("Unable to convert {} into a diet", value)),
        }
    }
}

impl Into<String> for Diet {
    fn into(self) -> String {
        match self {
            Diet::Omnivore => "omnivore",
            Diet::Vegetarian => "vegetarian",
            Diet::Vegan => "vegan",
        }
        .to_string()
    }
}

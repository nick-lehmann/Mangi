pub enum MenuProperties {
    Meat,
    Vegetarian,
    Vegan,
    Wheat,
    Gluten,
    Sesam,
}

impl MenuProperties {
    pub fn from_tu_dresden(notes: &Vec<String>) -> Vec<Self> {
        notes
            .iter()
            .map(|note| match note.as_str() {
                "Menü ist vegan" => MenuProperties::Vegan,
                _ => todo!(),
            })
            .collect()
    }
}

use crate::internal::mensa::models::Diet;

pub fn get_diet_from_tu_dresden(notes: &Vec<String>) -> Diet {
    for note in notes {
        if note == "Menü ist vegan" {
            return Diet::Vegan;
        }
        if note == "Menü ist vegetarisch" {
            return Diet::Vegetarian;
        }
    }

    Diet::Omnivore
}

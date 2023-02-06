pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

const ALL_ALLERGEN: [Allergen; 8] = [
    Allergen::Eggs,
    Allergen::Peanuts,
    Allergen::Shellfish,
    Allergen::Strawberries,
    Allergen::Tomatoes,
    Allergen::Chocolate,
    Allergen::Pollen,
    Allergen::Cats,
];

// This was a nice intro to the From trait but I don't understand why
// this is actually needed. It should be possible to convert the Enum
// "discriminator" to an i32 value.

impl From<&Allergen> for u32 {
    fn from(v: &Allergen) -> u32 {
        match v {
            Allergen::Eggs => Allergen::Eggs as u32,
            Allergen::Peanuts => Allergen::Peanuts as u32,
            Allergen::Shellfish => Allergen::Shellfish as u32,
            Allergen::Strawberries => Allergen::Strawberries as u32,
            Allergen::Tomatoes => Allergen::Tomatoes as u32,
            Allergen::Chocolate => Allergen::Chocolate as u32,
            Allergen::Pollen => Allergen::Pollen as u32,
            Allergen::Cats => Allergen::Cats as u32,
        }
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergen_index: u32 = allergen.into();
        self.score & allergen_index != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        ALL_ALLERGEN
            .into_iter()
            .filter(|allergen| self.is_allergic_to(allergen))
            .collect()
    }
}

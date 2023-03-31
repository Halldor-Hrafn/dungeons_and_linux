use std::collections::HashMap;

// todo: organize this shit
pub struct PlayableCharacter {
    first_name: String,
    surname: String,
    age: u16,
    traits: String,
    max_health: u16,
    level: u8,
    class: String,
    stats: Vec<HashMap>,
    proficiency_bonus: u8,
    proficiencies: Vec<String>,
    other_proficiencies: Vec<String>,
    languages: Vec<String>,
    equipment: Vec<String>,
    coins: Vec<HashMap>,
    personality: String,
    ideals: String,
    bonds: String,
    flaws: String,
}

impl PlayableCharacter {
    pub fn new() -> Self {
        // todo: find a way to simplify picking random names
        // todo: find a way to generate a class with realistic stats
        // todo: find a way to generate random equipments and proficiencies
        // todo: find a way to generate random ideals, bonds, flaws, etc.

        // maybe add a lib.rs and some traits once I've worked more on this
    }
}
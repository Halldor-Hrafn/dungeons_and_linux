use std::collections::HashMap;

// todo: organize this shit
pub struct PlayableCharacter {
    first_name: String,
    surname: String,
    age: u16,
    heigth: u16,
    weight: u16,
    eyes: String,
    skin: String,
    hair: String,
    traits: Vec<String>,
    features: Vec<String>,
    max_health: u16,
    level: u8,
    class: String,
    sub_class: String,
    stats: HashMap<String, u8>,
    proficiency_bonus: u8,
    proficiencies: Vec<String>,
    other_proficiencies: Vec<String>,
    languages: Vec<String>,
    equipment: Vec<String>,
    coins: HashMap<String, u16>,
    personality: String,
    ideals: String,
    bonds: String,
    flaws: String,
    faction_affiliation: Vec<String>,
    backstory: String,
    spellcasting_ability: String,
    spell_save_dc: u8,
    spell_attack_bonus: u8,
    spells: HashMap<String, u8>
}

impl PlayableCharacter {
    pub fn new() -> Self {
        // todo: find a way to simplify picking random names
        // todo: find a way to generate a class with realistic stats
        // todo: find a way to generate random equipments and proficiencies
        // todo: find a way to generate random ideals, bonds, flaws, etc.
        // todo: find a way to generate random spells
        // todo: find a way to do all of this shit

        // maybe add a lib.rs and some traits once I've worked more on this
        todo!()
    }
}
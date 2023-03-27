use std::env;
use rand::{self, Rng};

#[derive(Debug)]
struct Character {
    first_name: String,
    surname: String,
    age: u16,
    traits: String,
}

impl Character {
    fn new() -> Self {
        let mut rng = rand::thread_rng();

        let male_names = [
            "Houn",
            "Rhivaun",
            "Umbril",
            "Xaemar",
            "Zeltaebar",
            "Darvin",
            "Dorn",
            "Evendur",
            "Gorstag",
            "Grim",
            "Helm",
            "Malark",
            "Morn",
            "Randal",
            "Stedd",
            "Theronius",
            "Varianth",
            "Jarekai",
            "Elendrian",
            "Arcturius"         
        ];

        let female_names = [
            "Glouris",
            "Maeve",
            "Sevaera",
            "Xaemarra",
            "Zraela",
            "Araveen",
            "Asvele",
            "Jhessail",
            "Kerri",
            "Lureene",
            "Miri",
            "Rowan",
            "Shandri",
            "Tessele",
            "Lyrastra",
            "Caelondra",
            "Torrinthia",
            "Elendrian",
            "Sylvarian",
            "Tavionna"
        ];

        let surnames = [
            "Lharaendo",
            "Mristar",
            "Wyndael",
            "Amblecrown",
            "Buckman",
            "Dundragon",
            "Evenwood",
            "Greycastle",
            "Tallstag"
        ];

        let traits = [
            "Immortal",
            "Shapeshifter",
            "Deity"
        ];

        let pick_gender = rng.gen_bool(0.5);

        if pick_gender {
            return Self {
                first_name: male_names[rng.gen_range(0..20)].to_string(),
                surname: surnames[rng.gen_range(0..9)].to_string(),
                age: rng.gen_range(1..750),
                traits: traits[rng.gen_range(0..3)].to_string(),
            }
        } else {
            return Self {
                first_name: female_names[rng.gen_range(0..20)].to_string(),
                surname: surnames[rng.gen_range(0..9)].to_string(),
                age: rng.gen_range(1..750),
                traits: traits[rng.gen_range(0..3)].to_string(),
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = &args[1];

    if command == "character" {
        let character = Character::new();

        println!("{:#?}", character);
    }

    // dbg!(args);
}

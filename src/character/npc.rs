use rand::{self, Rng};

#[derive(Debug)]
pub struct Npc {
    first_name: String,
    surname: String,
    age: u16,
    traits: String,
}

impl Npc {
    pub fn new() -> Self {
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
            "Arcturius",
            "Eldrin",
            "Rendrick",
            "Andros",
            "Lucien",
            "Erevan",
            "Corin",
            "Callum",
            "Gideon",
            "Lirien",
            "Darian"
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
            "Tavionna",
            "Alianor",
            "Ilmare",
            "Gwynneth",
            "Amalthea",
            "Anora",
            "Aurielle",
            "Belisent",
            "Eillistraee",
            "Seraphine",
            "Zephyrine"
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
            "Tallstag",
            "Amalith",
            "Valtairis",
            "Danilor",
            "Eldrinthal",
            "Ilmareen",
            "Corinthal",
            "Gwynnethian",
            "Lirienhart",
            "Erevanthir",
            "Sarienwood"
        ];

        let traits = [
            "Immortal",
            "Shapeshifter",
            "Deity",
            "Brave",
            "Charismatic",
            "Resilient",
            "Skilled",
            "Intelligent",
            "Agile",
            "Perceptive",
            "Resourcful",
            "Faithful",
            "Ambitious"
        ];

        let pick_gender = rng.gen_bool(0.5);

        if pick_gender {
            return Self {
                first_name: male_names[rng.gen_range(0..male_names.len())].to_string(),
                surname: surnames[rng.gen_range(0..surnames.len())].to_string(),
                age: rng.gen_range(1..750),
                traits: traits[rng.gen_range(0..traits.len())].to_string(),
            }
        } else {
            return Self {
                first_name: female_names[rng.gen_range(0..female_names.len())].to_string(),
                surname: surnames[rng.gen_range(0..surnames.len())].to_string(),
                age: rng.gen_range(1..750),
                traits: traits[rng.gen_range(0..traits.len())].to_string(),
            }
        }
    }
}

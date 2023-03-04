use std::env;
use rand::{self, Rng};

// fn gen_rand_int(range_1: u8, range_2: u8) -> u8 {
//     rand::thread_rng().gen_range(range_1..range_2)
// }

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = &args[1];

    if command == "name" {
        let name = name_handler();

        println!("{}", name);
    }

    // dbg!(args);
}

fn name_handler() -> String {
    let male_names = [
        "Houn",
        "Rhivaun",
        "Umbril",
        "Xaemar",
        "Zeltaebar"
    ];

    let female_names = [
        "Glouris",
        "Maeve",
        "Sevaera",
        "Xaemarra",
        "Zraela"
    ];

    let surnames = [
        "Lharaendo",
        "Mristar",
        "Wyndael"
    ];

    let name: String = format!("{} {}", &male_names[rand::thread_rng().gen_range(0..4)], &surnames[rand::thread_rng().gen_range(0..2)]);

    return name;
}

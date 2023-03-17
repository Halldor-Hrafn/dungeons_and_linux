use std::env;
use rand::{self, Rng};

// fn gen_rand_int(range_1: u8, range_2: u8) -> u8 {
//     rand::thread_rng().gen_range(range_1..range_2)
// }

// fn pick_rand_value(first_name: Vec<&String>, surname: Vec<&String>) -> String {
//     let name = format!()
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
    let mut rng = rand::thread_rng();
    let name: String;

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

    // let pick_gender = rand::thread_rng().gen_bool(0.5);
    let pick_gender = &rng.gen_bool(0.5);

    if *pick_gender {
        name = format!("{} {}", &male_names[rng.gen_range(0..5)], &surnames[rng.gen_range(0..3)]);
    } else {
        name = format!("{} {}", &female_names[rng.gen_range(0..5)], &surnames[rng.gen_range(0..3)]);
    }

    return name;
}

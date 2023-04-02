use std::env;

use crate::character::npc::Npc;

pub mod character;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = &args[1];

    if command == "npc" {
        let character = Npc::new();

        println!("{:#?}", character);
    }

    // dbg!(args);
}

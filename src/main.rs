use std::io;
use std::collections::HashMap;
use std::ops::Add;

fn main() {
    println!("Welcome to Counting Chars!");
    println!();
    println!("Input the text you want to analyse:");

    let min_amount = 2;

    let mut minority_name: String = String::new();
    let mut minority_amount: u32 = 0;

    let mut other_amount: u32 = 0;

    let mut map: HashMap<String, u32> = HashMap::new();
    let mut input: String = String::new();
    io::stdin().read_line(&mut input);

    for char in input.chars() {
        if char as u16 >= 32 && char as u16 <= 126 {
            map.insert(char.to_string(), (if map.contains_key(&char.to_string()) { map[&char.to_string()] } else { 0 }) + 1);
        } else {
            other_amount += 1;
        }
    }

    for (c, amount) in map {
        if amount >= min_amount {
            println!("\"{}\": {}", c, amount);
        } else {
            minority_name = format!("{}\"{}\" ", minority_name, c.to_string());
            minority_amount += amount;
        }
    }

    if minority_amount != 0 {
        println!("{}: {}", minority_name, minority_amount);
    }

    if other_amount != 0 {
        println!("Other: {}", other_amount);
    }

    println!();
    println!("END!");

}

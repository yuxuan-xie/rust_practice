use std::collections::HashMap;
use std::{io};

pub fn front() -> i32 {
    println!("Welcome to the interface system.");
    println!("Enter an instruction to manipulate the name table.");
    println!("Enter list to print the table.");
    println!("Enter quit to shut down.");

    let mut name_table: HashMap<String, Vec<String>> = HashMap::new();

    while handle_input(&mut name_table) {}

    0
}

fn handle_input(name_table: &mut HashMap<String, Vec<String>>) -> bool {
    let instru_collection = ["add"];

    let mut instruction = String::new();

    io::stdin()
        .read_line(&mut instruction)
        .expect("Fail to read line.");

    let instruction = instruction.split_whitespace().collect::<Vec<&str>>();
    let length = instruction.len();
    let mut op = String::new();
    let mut name = String::new();
    let mut depart = String::new();

    match length {
        1 => {
            if instruction[0] == "quit" {
                return false;
            } else if instruction[0] == "list" {
                println!("{:?}", name_table);
                return true;
            }
            println!("Illegal input.");
            return true;
        }
        4 => {
            if instru_collection.contains(&instruction[0]) {
                op.push_str(instruction[0]);
                name.push_str(instruction[1]);
                depart.push_str(instruction[3]);
            } else {
                println!("Illegal input.");
                return true;
            }
        }
        _ => {
            println!("Illegal input.");
            return true;
        }
    }

    let v = name_table.entry(depart).or_insert(Vec::<String>::new());
    if op == "add".to_string(){
        v.push(name);
    }

    true
}

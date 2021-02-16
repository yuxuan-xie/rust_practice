use std::io;

pub mod utils;

fn main() {
    utils::print_func_list();

    loop {
        println!("Please select a function[or exit to shut down]:");

        let mut func_id = String::new();

        io::stdin()
            .read_line(&mut func_id)
            .expect("Fail to read line.");

        if func_id.trim() == "exit" {
            break;
        }

        let func_id: i32 = match func_id.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match func_id {
            0 => utils::print_func_list(),
            1 => println!("Return value:{}", utils::c2f()),
            2 => println!("Return value:{}", utils::fibonacci()),
            3 => println!("Return value:{}", utils::bubble_sort()),
            _ => continue,
        }
    }
}

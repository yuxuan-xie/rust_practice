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
            4 => println!("Return value:{}", utils::quick_sort()),
            5 => println!("Return value:{}", utils::quick_sort_slice()),
            6 => println!("Return value:{}", utils::pig_latin()),
            7 => println!("Return value:{}", utils::department_interface::front()),
            8 => println!("Return value:{}", utils::enter_merge()),
            9 => println!("Return value:{}", utils::insertion_sort()),
            10 => println!("Return value:{}", utils::maximum_subarray()),
            11 => println!("Return value:{}", utils::maximum_subarray_brute_force()),
            12 => println!("Return value:{}", utils::find_i()),
            13 => println!("Return value:{}", utils::heap_sort()),
            _ => continue,
        }
    }
}

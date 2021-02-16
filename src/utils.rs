use std::{io, vec};

pub fn print_func_list() {
    let funcs = ["print this list", "c2f", "fibonacci", "bubble sort"];

    for (index, each) in funcs.iter().enumerate() {
        println!("{}:{}", index, each);
    }
}

pub fn c2f() -> i32 {
    return loop {
        println!("Please input a celsius temperature[or quit to shut down]:");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Fail to read line.");
        if input.trim() == "quit" {
            break 0;
        }

        let input: f64 = match input.trim().parse() {
            Ok(float) => float,
            Err(_) => break -1,
        };

        let output: f64 = 9.0 / 5.0 * input + 32.0;

        println!("Celsius:{}\tFahrenheit:{}", input, output);
    };
}

pub fn fibonacci() -> i32 {
    return loop {
        println!("Please enter the value for n[or quit to shut down]:");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Fail to read line.");

        if input.trim() == "quit" {
            break 0;
        }

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => break -1,
        };

        println!("the {}th Fibonacci number is:{}", input, fib_rec(input));
    };
}

pub fn fib_rec(x: u32) -> u32 {
    match x {
        0 => 0,
        1 => 1,
        2 => 1,
        _t => fib_rec(x - 1) + fib_rec(x - 2),
    }
}

pub fn bubble_sort() -> i32{
    let mut v = vec![32, 43, 54, 22, 32, 10, 1, 99, 33, 0];
    let length = v.len();
    println!("Raw array: {:?}", v);
    for i in 0..length - 1{
        for j in 0..length - 1 - i{
            if &v[j] > &v[j+1]{
                let temp = v[j];
                *&mut v[j] = v[j+1];
                *&mut v[j+1] = temp;

                println!("iteration({}, {}):{:?}", i, j, v);
            }
        }
    }
    return 0;
}
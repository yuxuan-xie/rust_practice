use std::io;

fn main(){
   
    print_func_list();

    loop{
        println!("Please select a function[or exit to shut down]:");

        let mut func_id = String::new();

        io::stdin()
            .read_line(&mut func_id)
            .expect("Fail to read line.");

        if func_id.trim() == "exit" {break;}

        let func_id : i32 = match func_id.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match func_id{
            0 => print_func_list(),
            1 => println!("Return value:{}", c2f()),
            2 => println!("Return value:{}", fibonacci()),
            _ => continue,
        }
    }
}

fn print_func_list(){
    let funcs = ["print this list", "c2f", "fibonacci"];

    let mut index = 0;
    for each in funcs.iter(){
        println!("{}:{}", index, each);
        index += 1;
    }
}

fn c2f() -> i32 {
    return loop{
        println!("Please input a celsius temperature[or quit to shut down]:");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Fail to read line.");

        if input.trim() == "quit" {break 0;}

        let input : f64 = match input.trim().parse() {
            Ok(float) => float,
            Err(_) => break -1,
        };

        let output : f64 = 9.0/5.0 * input + 32.0;

        println!("Celsius:{}\tFahrenheit:{}", input, output);
    };
}

fn fibonacci() -> i32{
    return loop{
        println!("Please enter the value for n[or quit to shut down]:");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Fail to read line.");

        if input.trim() == "quit" {break 0;}

        let input: i32 = match input.trim().parse(){
            Ok(num) => num,
            Err(_) => break -1,
        };

        println!("the {}th Fibonacci number is:{}", input, fib_rec(input));
    };
}

fn fib_rec(x:i32) -> i32{
    if x == 1 {return 1;}
    
    if x == 2 {return 1;}

    return fib_rec(x-1) + fib_rec(x-2);
}
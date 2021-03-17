use std::io;

pub fn print_func_list() {
    let funcs = [
        "print this list",
        "c2f",
        "fibonacci",
        "bubble sort",
        "quick sort",
        "quick sort slice",
        "pig latin",
        "department interface",
        "merge sort",
        "insertion sort",
        "maximum subarray",
        "maximum subarray brute force",
        "find_i",
        "heap_sort",
    ];

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
        _ => fib_rec(x - 1) + fib_rec(x - 2),
    }
}

pub fn bubble_sort() -> i32 {
    let mut v = vec![32, 43, 54, 22, 32, 10, 1, 99, 33, 0];
    let length = v.len();
    println!("Raw array: {:?}", v);
    for i in 0..length - 1 {
        for j in 0..length - 1 - i {
            if &v[j] > &v[j + 1] {
                let temp = v[j];
                *&mut v[j] = v[j + 1];
                *&mut v[j + 1] = temp;

                println!("iteration({}, {}):{:?}", i, j, v);
            }
        }
    }
    return 0;
}

pub fn quick_sort() -> i32 {
    let mut v = vec![32, 43, 54, 22, 32, 10, 1, 99, 33, 0];
    println!("Raw array: {:?}", v);
    let left = 0;
    let right = v.len() - 1;
    iter_quick_sort(&mut v, left, right);

    println!("Sorted: {:?}", v);
    0
}

fn iter_quick_sort(v: &mut Vec<i32>, left: usize, right: usize) {
    if left >= right {
        return;
    }

    let mut i = left;
    let mut j = right;
    let key = v[left];

    while i < j {
        while i < j && key <= v[j] {
            j -= 1;
        }
        *&mut v[i] = v[j];
        println!("Iteration({}, {}): {:?}", i, j, v);

        while i < j && key >= v[i] {
            i += 1;
        }
        *&mut v[j] = v[i];
        println!("Iteration({}, {}): {:?}", i, j, v);
    }

    *&mut v[i] = key;
    if i > 1 {
        // Since i and j are defined as usize, pay attention to the case in which i - 1 might causes overflow.
        iter_quick_sort(v, left, i - 1);
    }
    iter_quick_sort(v, i + 1, right);
}

pub fn quick_sort_slice() -> i32 {
    let mut v = vec![32, 43, 54, 22, 32, 10, 1, 99, 33, 0];
    println!("Raw array: {:?}", v);

    iter_quick_sort_slice(&mut v[..]);

    println!("Sorted: {:?}", v);
    0
}

fn iter_quick_sort_slice(v: &mut [i32]) {
    if v.len() == 0 {
        return;
    }

    let mut i: usize = 0;
    let mut j: usize = v.len() - 1;
    let key: i32 = match v.get(i) {
        Some(num) => *num,
        None => return,
    };

    while i < j {
        while i < j && key <= v[j] {
            j -= 1;
        }
        *&mut v[i] = v[j];
        // println!("{:?}", v);

        while i < j && key >= v[i] {
            i += 1;
        }
        *&mut v[j] = v[i];
        // println!("{:?}", v);
    }
    *&mut v[i] = key;
    println!("{:?}", v);

    if i >= 1 {
        iter_quick_sort_slice(&mut v[..i]);
    }
    iter_quick_sort_slice(&mut v[i + 1..]);
}

pub fn pig_latin() -> i32 {
    let vowel = vec!['a', 'e', 'i', 'o', 'u'];

    let mut input = vec![
        "apple".to_string(),
        "first".to_string(),
        "second".to_string(),
        "perfunctory".to_string(),
        "heterogenuous".to_string(),
        "ubiquitous".to_string(),
        "egregious".to_string(),
        "ostentatious".to_string(),
        "panacea".to_string(),
        "trepidation".to_string(),
    ];

    println!("Raw: {:?}", input);

    for word in &mut input {
        let first_char: char = match word.chars().next() {
            Some(ch) => ch,
            None => return -1,
        };

        if vowel.contains(&first_char) {
            word.push_str("-hay");
            continue;
        }

        word.push('-');
        word.push(first_char);
        word.push_str("ay");

        word.remove(0);
    }

    println!("Result: {:?}", input);
    0
}

pub mod department_interface;

pub fn enter_merge() -> i32 {
    let mut v = vec![32, 43, 54, 22, 32, 10, 1, 99, 33, 0, 0, 23, 44, 98, 53];

    merge(&mut v[..]);

    println!("{:?}", v);
    0
}

fn merge(a: &mut [i32]) {
    let length = a.len();

    if length > 2 {
        merge(&mut a[..length / 2]);
        merge(&mut a[length / 2..]);
    }
    merge_sort(a);
    // println!("{:?}", a);
}

fn merge_sort(a: &mut [i32]) {
    let mid = a.len() / 2;
    let mut lvec: Vec<i32> = Vec::new();
    let mut rvec: Vec<i32> = Vec::new();
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < mid {
        lvec.push(a[i]);
        i += 1;
    }
    while mid + j < a.len() {
        rvec.push(a[mid + j]);
        j += 1;
    }

    i = 0;
    j = 0;

    while k < a.len() {
        if i == mid {
            a[k] = rvec[j];
            j += 1;
        } else if j == mid {
            a[k] = lvec[i];
            i += 1;
        } else if lvec[i] <= rvec[j] {
            a[k] = lvec[i];
            i += 1;
        } else {
            a[k] = rvec[j];
            j += 1;
        }
        k += 1;
    }
}

pub fn insertion_sort() -> i32 {
    let mut v = vec![32, 43, 54, 22, 32, 10, 1, 99, 33, 0, 0, 23, 44, 98, 53];
    let mut i: i32 = 1;
    let mut j;
    let mut key: i32;
    while i < v.len() as i32 {
        key = v[i as usize];
        j = i - 1;
        while j >= 0 && key < v[j as usize] {
            v[(j + 1) as usize] = v[j as usize];
            j -= 1;
        }
        v[(j + 1) as usize] = key;
        i += 1;
    }
    println!("{:?}", v);
    return 0;
}

pub fn maximum_subarray() -> i32 {
    let v = vec![
        13, -3, -25, 30, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7,
    ];
    let ret = find_maximum_subarray(&v, 0 as usize, v.len() - 1);
    println!("sequence:{:?}", v);
    println!("ans:{:?}", ret);
    return 0;
}

fn find_maximum_subarray(v: &Vec<i32>, low: usize, high: usize) -> (usize, usize, i32) {
    // println!("{}, {}", low, high);
    if low == high {
        return (low, high, 0);
    }

    let mid = (low + high) / 2;
    let lmax: (usize, usize, i32) = find_maximum_subarray(v, low, mid);
    let rmax: (usize, usize, i32) = find_maximum_subarray(v, mid + 1, high);
    let cmax: (usize, usize, i32) = find_maximum_cross(v, low, high, mid);

    if lmax.2 > rmax.2 && lmax.2 > cmax.2 {
        return lmax;
    } else if rmax.2 > lmax.2 && rmax.2 > cmax.2 {
        return rmax;
    } else {
        return cmax;
    }
}

fn find_maximum_cross(v: &Vec<i32>, low: usize, high: usize, mid: usize) -> (usize, usize, i32) {
    // println!("{}, {}, {}", low, high, mid);
    let mut i = mid;
    let mut sum = 0;
    let mut lmax = -9999999;
    let mut ret: (usize, usize, i32) = (0, 0, 0);
    while i >= low {
        sum += v[i];
        if sum > lmax {
            lmax = sum;
            ret.0 = i;
        }
        if i == 0 {
            break;
        }
        i -= 1;
    }

    let mut rmax = -99999999;
    sum = 0;
    i = mid + 1;
    while i <= high {
        sum += v[i];
        if sum > rmax {
            rmax = sum;
            ret.1 = i;
        }
        i += 1;
    }

    ret.2 = lmax + rmax;

    return ret;
}

pub fn maximum_subarray_brute_force() -> i32 {
    let v = vec![
        13, -3, -25, 30, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7,
    ];
    let mut sum;
    let mut ret: (usize, usize, i32) = (0, 0, -9999);

    for i in 0..v.len() {
        sum = v[i];
        if sum >= ret.2 {
            ret = (i, i, sum);
        }
        for j in i + 1..v.len() {
            sum += v[j];
            if sum >= ret.2 {
                ret = (i, j, sum);
            }
        }
    }
    println!("ans:{:?}", ret);

    return 0;
}

pub fn find_i() -> i32 {
    let v = vec![-10, -4, -3, -2, 0, 5, 8, 10, 14, 45, 66, 77, 88];

    println!("ans:{}", find_i_rec(&v, 0, v.len()));

    return 0;
}

fn find_i_rec(v: &Vec<i32>, low: usize, high: usize) -> usize {
    let mid = (low + high) / 2;
    // println!("{}", mid);
    if (mid as i32) == v[mid] {
        return mid;
    } else if (mid as i32) < v[mid] {
        return find_i_rec(v, low, mid);
    } else {
        return find_i_rec(v, mid + 1, high);
    }
}

fn siftup(v: &mut Vec<i32>, i: usize, n: usize) {
    let mut j: usize = i * 2 + 1;
    let mut i = i;
    let t = v[i];

    loop{
        if j >= n{
            break;
        }        

        if j+1 < n && v[j] < v[j + 1]{
            j += 1;
        }
        if t < v[j]{
            v[i] = v[j];
            i = j;
        }else{
            break;
        }

        j = i * 2 + 1;
    }

    v[i] = t;
}

pub fn heap_sort() -> i32 {
    let mut v = vec![32, 43, 54, 22, 32, 10, 1, 99, 33, 0, 0, 23, 44, 98, 53];
    let mut i = v.len() / 2 - 1;
    let n = v.len();

    println!("{:?}", v);
    loop{
        siftup(&mut v, i, n);
        if i == 0 {
            break;
        }
        i -= 1;
    }

    i = v.len() - 1;
    while i > 0 {
        let t = v[i];
        v[i] = v[0];
        v[0] = t;
        siftup(&mut v, 0, i);
        i -= 1;
    }
    println!("{:?}", v);
    return 0;
}
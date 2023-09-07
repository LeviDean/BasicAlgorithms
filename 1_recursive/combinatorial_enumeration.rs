// https://oj.haizeix.com/problem/236

use std::io;

fn f(i: i32, j: i32, n: i32, m: i32, arr: &mut Vec<i32>) {
    if arr.len() == m as usize {
        for item in arr.iter() {
            print!("{} ", item);
        }
        println!("");
    } else {
        for k in j..n + 1 {
            let mut tmp = arr.clone();
            tmp.push(k);
            f(i + 1, k + 1, n, m, &mut tmp);
        }
    }
}


fn main() {
    println!("Please input a integer n: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: i32 = input.trim().parse().expect("Please type a integer number!");

    println!("Please input another integer m: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let m: i32 = input.trim().parse().expect("Please type a integer number!");
    
    let mut arr = Vec::new();

    f(0, 1, n, m, &mut arr);

}
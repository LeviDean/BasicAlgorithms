// https://oj.haizeix.com/problem/186

use std::io;

fn jump(n: &i32, k: i32, arr: &Vec<i32>) -> i32 {
    if k + arr[k as usize] >= *n {
        1
    } else {
        1 + jump(n, k + arr[k as usize], arr)
    }
}

fn main() {
    println!("Please input a integer: ");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Please type a integer number!");

    println!("Please input a integer sequence: ");
    input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Please type a integer number!"))
        .collect();

    // get jump times
    let times = jump(&n, 0, &nums);

    println!("The total jump times is: {}", times)
}

// https://oj.haizeix.com/problem/237


use std::io;

fn f(i: i32, n: i32, nums: &Vec<i32>, arr: &mut Vec<i32>) {
    if i > n {
        for item in arr.iter() {
            print!("{} ", item);
        }
        println!("");
        return;
    } else {
        for num in nums.iter() {
            let mut new_arr = arr.clone();
            let mut new_nums = nums.clone();
            new_arr.push(*num);
            new_nums.retain(|&x| x != *num);
            f(i + 1, n, &new_nums, &mut new_arr);
        }
    }
}

fn main(){

    println!("Please input a integer n: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Please type a integer number!");

    let nums: Vec<i32> = (1..=n).collect();
    let mut arr = Vec::new();
    f(1, n, &nums, &mut arr)
}
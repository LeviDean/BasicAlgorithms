// https://oj.haizeix.com/problem/235


use std::io;
use std::collections::HashSet;

fn generate_sequence(k: i32, n: i32, set: &mut HashSet<Vec<i32>>) {
    if k == 1 {
        set.insert(vec![1]);
    } else {
        generate_sequence(k-1, n, set);
        let mut new_set = HashSet::new();
        new_set.insert(vec![k]);
        for item in set.iter() {
            let mut new_vec = item.clone();
            new_vec.push(k);
            new_set.insert(new_vec);
        }
        set.extend(new_set);
    }
}


fn f(i: i32, j: i32, n: i32, arr: &mut Vec<i32>) {
    /* 
    if j > n {
        return;
    } else {
        f(i, j, n) = 
        0       ->  j + 0       +   f(i + 1, j + 1, n)
        1       ->  j + 1       +   f(i + 1, j + 2, n)
        ....
        n - j   ->  j + n - j   +   f(i + 1, n + 1, n)
    }
    */
    if j > n {
        return; 
    } else {
        for k in j..n+1 {
            arr[i as usize] = k;
            for n in arr.iter() {
                if *n != 0 {
                    print!("{} ", n);
                } 
            }
            println!("");
            let mut new_arr = arr.clone();
            f(i + 1, k + 1, n, &mut new_arr);
        }
    }
}

fn main() {

    println!("Please input a integer: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: i32 = input.trim().parse().expect("Please type a integer number!");

    let mut set = HashSet::new();
    generate_sequence(n, n, &mut set);
    for v in set.iter() {
        println!("{:?}", v);
    }


    // a better method
    println!("A better method:");
    let mut test = vec![0; n as usize];
    f(0,1,n, &mut test);
}
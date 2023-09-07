// 题目描述
// 路飞买了一堆桃子不知道个数，第一天吃了一半的桃子，还不过瘾，又多吃了一个。
// 以后他每天吃剩下的桃子的一半还多一个，到 n 天只剩下一个桃子了。路飞想知道一开始买了多少桃子。

// 输入
// 输入一个整数 n(2≤n≤30)

// 输出
// 输出买的桃子的数量。

use std::io;

fn rest_peach(k: i32) -> i32 {
    if k == 1 {
        1
    } else {
        (rest_peach(k-1) + 1) * 2
    }
}

fn main() {
    // get n from keyboard
    println!("Please input a integer: ");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: i32 = input
        .trim()
        .parse()
        .expect("Please type a integer number!");

    // calculate the rest peach
    let total = rest_peach(n);

    println!("The total peach is: {}", total);
}

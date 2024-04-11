use std::io;

fn min_cost(s: &str, a: i32, b: i32) -> i32 {
    let mut balance = 0;
    let mut swaps = 0;
    let mut cost = 0;

    for c in s.chars() {
        if c == '(' {
            balance += 1;
        } else {
            if balance == 0 {
                swaps += 1;
            } else {
                balance -= 1;
            }
        }
    }

    // 最小コストを計算
    cost += std::cmp::min(swaps * a, (swaps + balance) * b);
    cost
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let n = parts[0];
    let a = parts[1];
    let b = parts[2];

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let s = s.trim();

    let result = min_cost(s, a, b);
    println!("{}", result);
}

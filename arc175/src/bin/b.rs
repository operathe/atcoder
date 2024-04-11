use std::io;

fn min_cost_to_make_valid_parentheses(s: &str, a: i32, b: i32) -> i32 {
    let mut cost = 0;
    let mut balance = 0;

    for c in s.chars() {
        if c == '(' {
            balance += 1;
        } else if balance > 0 {
            balance -= 1;
        } else {
            cost += std::cmp::min(a, b);
        }
    }

    cost + balance * b
}

fn main() {
    let mut input = String::new();

    // N, A, B を読み込む
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let (_n, a, b) = (parts[0], parts[1], parts[2]);

    // S を読み込む
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let s = input.trim();

    let result = min_cost_to_make_valid_parentheses(s, a, b);
    println!("{}", result);
}

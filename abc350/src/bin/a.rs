use proconio::marker::Chars;
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let n = s[3..].iter().collect::<String>().parse::<i64>().unwrap();
    println!("{}", if check(n) { "Yes" } else { "No" });
}

fn check(n: i64) -> bool {
    if n == 316 || n >= 350 || n == 0 {
        return false;
    } else {
        return true;
    }
}

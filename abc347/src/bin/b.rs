use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        s: String,
    }

    let mut substrings = HashSet::new();

    for start in 0..s.len() {
        for end in start + 1..=s.len() {
            substrings.insert(&s[start..end]);
        }
    }

    println!("{}", substrings.len());
}

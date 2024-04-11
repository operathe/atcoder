use std::collections::HashMap;
use std::io;

fn count_swaps(s: &str) -> i64 {
    let n = s.len() as i64;
    let total_pairs = n * (n - 1) / 2; // 全ての異なるペアの数

    // 同じ文字のペアの数を減算する
    let mut char_counts = HashMap::new();
    for ch in s.chars() {
        *char_counts.entry(ch).or_insert(0) += 1;
    }
    let same_char_pairs: i64 = char_counts.values().map(|&k| k * (k - 1) / 2).sum();

    // 異なる文字のペアの数
    let different_char_pairs = total_pairs - same_char_pairs;
    if *char_counts.values().max().unwrap_or(&0) > 1 {
        different_char_pairs + 1
    } else {
        different_char_pairs
    }
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let s = s.trim();

    println!("{}", count_swaps(s));
}

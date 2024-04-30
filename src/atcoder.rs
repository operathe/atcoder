use proconio::{fastout, input, marker::*};
#[fastout]
fn main() {
    input! {
        a: i64, b: i64,
    }
    // a個の]を配列に格納
    let mut vec_a = vec![];
    for _ in 0..a {
        vec_a.push(']');
    }
    // b個の]を配列に格納
    let mut vec_b = vec![];
    for _ in 0..b {
        vec_b.push(']');
    }
    println!("A:{}", vec_a.iter().collect::<String>());
    println!("B:{}", vec_b.iter().collect::<String>());
}

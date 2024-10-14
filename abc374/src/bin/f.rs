use proconio::{fastout, input};
use std::cmp::max;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        x: usize,
        t: [usize; n],
    }

    let dissatisfaction = simulate_shipping(&t, k, x);
    println!("{}", dissatisfaction);
}

fn simulate_shipping(t: &[usize], k: usize, x: usize) -> usize {
    let mut dissatisfaction = 0;
    let mut next_available_day = 0;
    let mut i = 0;

    while i < t.len() {
        // 次のバッチを出荷できる最も早い日を決定
        next_available_day = max(next_available_day, t[i]);

        // 最大K個の注文を出荷
        for _ in 0..k {
            if i < t.len() {
                // 現在の注文の不満度を計算
                dissatisfaction += next_available_day.saturating_sub(t[i]);
                i += 1;
            } else {
                break;
            }
        }

        // 出荷後、次の出荷可能日を更新
        next_available_day += x;
    }

    dissatisfaction
}

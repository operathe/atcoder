#![allow(unused_imports, unused_variables, dead_code, non_snake_case)]
use proconio::{fastout, input};

fn distance(x1: isize, y1: isize, x2: isize, y2: isize) -> f64 {
    (((x2 - x1) as f64).powi(2) + ((y2 - y1) as f64).powi(2)).sqrt()
}

#[fastout]
fn main() {
    input! {
        n: usize,
        s: f64,
        t: f64,
        abcd: [(isize, isize, isize, isize); n]
    }

    let mut total_time = 0.0;
    let mut current_x = 0;
    let mut current_y = 0;

    for &(a, b, c, d) in &abcd {
        // 現在位置から線分の両端点までの移動時間を計算
        let time_to_start = distance(current_x, current_y, a, b) / s;
        let time_to_end = distance(current_x, current_y, c, d) / s;

        // 近い方の端点を選択
        let move_time = time_to_start.min(time_to_end);

        // 描画時間を計算
        let draw_time = distance(a, b, c, d) / t;

        // 合計時間に加算
        total_time += move_time + draw_time;

        // 現在位置を更新（線分の終点に移動）
        current_x = c;
        current_y = d;
    }

    println!("{:.10}", total_time);
}

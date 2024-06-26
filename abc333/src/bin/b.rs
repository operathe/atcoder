#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::*};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::*;
#[allow(non_snake_case)]
#[allow(unused_variables)]
#[fastout]
fn main() {
    input! {
        s: Bytes,
        t: Bytes,
    }
    let d = |a: u8, b: u8| {
        let v = max(a, b) - min(a, b);
        v.min(5 - v) // ここを5から6に変更
    };
    let ans = if d(s[0], s[1]) == d(t[0], t[1]) {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}

#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::iter::FromIterator;
//スネークケースはない変数を許可する
#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        X: i64, A: i64, D : i64, N : i64
    }
    //例外として、N=0の場合はX-Aの絶対値を出力する
    if N == 0 {
        println!("{}", (X - A).abs());
        return;
    }
    let end = A + D * (N - 1);

    if X < min(A, end) {
        println!("{}", (min(A, end) - X).abs());
        return;
    }
    if X > max(A, end) {
        println!("{}", (X - max(A, end)).abs());
        return;
    }
    let min_operations = min_operations_to_good_number(A, D, N, X);
    println!("{}", min_operations);
}

#[allow(non_snake_case)]
fn min_operations_to_good_number(A: i64, D: i64, N: i64, X: i64) -> i64 {
    let mut low = 0;
    let mut high = N - 1;

    while low <= high {
        let mid = low + (high - low) / 2;
        let mid_val = A + D * mid;

        if mid_val == X {
            return 0;
        }

        if (D > 0 && mid_val < X) || (D < 0 && mid_val > X) {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    if low < N {
        min((A + D * low - X).abs(), (A + D * (low - 1) - X).abs())
    } else {
        (A + D * high - X).abs()
    }
}

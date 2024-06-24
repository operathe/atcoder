#[allow(unused_imports)]
use ac_library::ModInt998244353;
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
#[allow(unused_imports)]
use superslice::*;
#[allow(non_snake_case)]
#[allow(unused_variables)]
//type Mint = ModInt998244353;
#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut a_count = 0;
    let mut b_count = 0;
    let mut c_count = 0;
    for i in 0..n {
        if s[i] == 'A' {
            a_count += 1;
        } else if s[i] == 'B' {
            b_count += 1;
        } else if s[i] == 'C' {
            c_count += 1;
        }
        if a_count >= 1 && b_count >= 1 && c_count >= 1 {
            println!("{}", i + 1);
            return;
        }
    }
}

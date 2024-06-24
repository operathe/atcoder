#[allow(unused_imports)]
use ac_library::*;
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
type Mint = ModInt998244353;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Bytes,
    }
    let pt = 1 << (k - 1);
    let mut dp = vec![Mint::raw(0); pt];
    dp[0] = Mint::raw(1);
    for (i, &c) in s.iter().enumerate() {
        let mut ndp = vec![Mint::raw(0); pt];
        for (j, v) in dp.into_iter().enumerate() {
            let e = j.reverse_bits() >> (usize::BITS - k as u32);
            if c != b'B' && (i < k - 1 || e != j) {
                ndp[j >> 1] += v;
            }
            let e = (j | pt).reverse_bits() >> (usize::BITS - k as u32);
            if c != b'A' && (i < k - 1 || e != j | pt) {
                ndp[(j | pt) >> 1] += v;
            }
        }
        dp = ndp;
    }

    let mut ans = Mint::raw(0);
    for v in dp {
        ans += v;
    }
    println!("{ans}");
}

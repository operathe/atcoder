#[allow(unused_imports)]
use ac_library::ModInt1000000007;
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
type Mint = ModInt1000000007;
#[allow(non_snake_case)]
#[allow(unused_variables)]
#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut ans = Mint::new(1);
    let mut count = 1;
    for i in 1..s.len() {
        if s[i] == s[i - 1] {
            ans *= (count + 1) / 2;
            count = 1;
        } else {
            count += 1;
        }
    }
    ans *= (count + 1) / 2;
    println!("{}", ans);
}

// const MOD: usize = 1_000_000_007;
//
// fn main() {
//     input! {
//         n: usize,
//         mut s: Bytes,
//     }
//     let a = s.windows(2).map(|s| s[0] != s[1]).collect::<Vec<_>>();
//     let z = run_length_encoding(a);
//     let mut ans = 1;
//     for (a, b) in z {
//         if a {
//             ans = ans * (b / 2 + 1) % MOD;
//         }
//     }
//     println!("{}", ans);
// }
//
// fn run_length_encoding<T: Eq>(a: Vec<T>) -> Vec<(T, usize)> {
//     let mut a = a.into_iter().map(|a| (a, 1)).collect::<Vec<_>>();
//     a.dedup_by(|a, b| {
//         a.0 == b.0 && {
//             b.1 += a.1;
//             true
//         }
//     });
//     a
// }

#![allow(unused_imports, non_snake_case, unused_variables, dead_code)]
use ac_library::*;
use itertools::*;
use num_traits::pow;
use proconio::{fastout, input, marker::*};
use std::cmp::{max, min};
use std::collections::*;
use superslice::*;
type Mint = ModInt998244353;

#[fastout]

// fn main() {
//     input! {
//         s: Bytes,
//         t: Bytes,
//     }
//     for w in 1..s.len() {
//         for c in 0..w {
//             let mut q = vec![];
//             for i in (c..s.len()).step_by(w) {
//                 q.push(s[i]);
//             }
//             if q == t {
//                 println!("Yes");
//                 return;
//             }
//         }
//     }
//     println!("No");
// }
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    for i in 1..s.len() {
        let mut v = vec![vec![]; i];
        for j in 0..s.len() {
            v[j % i].push(s[j]);
        }
        for i in 0..i {
            if v[i] == t {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}

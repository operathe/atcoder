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
#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        a: i64,
        m: i64,
        l: i64,
        r: i64,
    }
    let geta = 10i64.pow(18) / m * m + m;
    let (l, r) = (l + geta + m, r + geta + m);
    let a = (a % m + m) % m;
    let x = (l - 1 - a) / m;
    let y = (r - a) / m;
    println!("{}", y - x);
}

// fn main() {
//     input! {
//         a: i64,
//         m: i64,
//         l: i64,
//         r: i64,
//     }
//     let x = (l - 1 - a).div_euclid(m);
//     let y = (r - a).div_euclid(m);
//     println!("{}", y - x);
// }

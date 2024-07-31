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
use superslice::*;
#[allow(non_snake_case)]
#[allow(unused_variables)]
const MOD: usize = 100_000_000;
#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [ usize; n],
    }
    a.sort_unstable();
    let sum = a.iter().sum::<usize>();
    let mut ans = sum * (n - 1);
    for (i, &x) in a.iter().enumerate() {
        let count = a[i + 1..].lower_bound(&(MOD - x));
        ans -= MOD * (n - 1 - i - count);
    }
    println!("{}", ans);
}

//     a.sort_unstable();
//     let mut ans = 0;
//     let mut j = n;
//     for (i, &x) in a.iter().enumerate() {
//         while j > 0 && a[j - 1] + x >= 100000000 {
//             j -= 1;
//         }
//         ans += x * (n - 1);
//         if i < j {
//             ans -= 50000000 * (n - j);
//         } else {
//             ans -= 50000000 * (n - j - 1);
//         }
//     }
//     println!("{}", ans);
// }

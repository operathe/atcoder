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
        a: [usize; 2*n],
    }
    let mut ans = 0;
    for i in 1..(2 * n - 1) {
        if a[i - 1] == a[i + 1] {
            ans += 1;
        }
    }
    println!("{}", ans);
}

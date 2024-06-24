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
        d: usize,
        s: [Bytes; n],
    }
    let mut ans = 0;
    for l in 0..d {
        let mut ok = true;
        for r in l..d {
            ok &= s.iter().all(|s| s[r] == b'o');
            if ok {
                ans = ans.max(r - l + 1);
            }
        }
    }
    println!("{}", ans);
}

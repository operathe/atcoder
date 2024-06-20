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
#[allow(non_snake_case)]
#[allow(unused_variables)]
type Mint = ModInt998244353;
#[fastout]
fn main() {
    input! {
        n: isize,
        s: Chars,
    }

    // Find the longest repetition of each character.
    let mut hi = vec![0; 26];
    let mut last = '-';
    let mut acc = 0;
    for c in s {
        let i = c as usize - 'a' as usize;
        if c == last {
            acc += 1;
        } else {
            last = c;
            acc = 1;
        }
        hi[i] = hi[i].max(acc);
    }

    // Find the sum as the answer.
    let ans = hi.iter().sum::<usize>();
    println!("{}", ans);
}

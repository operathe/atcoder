#![allow(unused_imports, unused_variables, dead_code, non_snake_case)]
use ac_library::*;
use itertools::*;
use num_traits::{abs, pow};
use proconio::{fastout, input, marker::*};
use std::cmp::{max, min};
use std::collections::*;
use superslice::*;

type Mint = ModInt998244353;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut s: Chars,
        xc: [(usize, char); q],
    }
    let mut count = count_abc(&s);
    for (x, c) in xc {
        let x = x - 1;
        if s[x] != c {
            // 変更前のカウント減少
            match (x > 0, x < n - 1) {
                (true, true) if s[x - 1] == 'A' && s[x] == 'B' && s[x + 1] == 'C' => count -= 1,
                _ => (),
            }
            match x > 1 {
                true if s[x - 2] == 'A' && s[x - 1] == 'B' && s[x] == 'C' => count -= 1,
                _ => (),
            }
            match x < n - 2 {
                true if s[x] == 'A' && s[x + 1] == 'B' && s[x + 2] == 'C' => count -= 1,
                _ => (),
            }
            s[x] = c;
            // 変更後のカウント増加
            match (x > 0, x < n - 1) {
                (true, true) if s[x - 1] == 'A' && s[x] == 'B' && s[x + 1] == 'C' => count += 1,
                _ => (),
            }
            match x > 1 {
                true if s[x - 2] == 'A' && s[x - 1] == 'B' && s[x] == 'C' => count += 1,
                _ => (),
            }
            match x < n - 2 {
                true if s[x] == 'A' && s[x + 1] == 'B' && s[x + 2] == 'C' => count += 1,
                _ => (),
            }
        }
        println!("{}", count);
    }
}

fn count_abc(s: &[char]) -> usize {
    s.windows(3).filter(|w| w == &['A', 'B', 'C']).count()
}

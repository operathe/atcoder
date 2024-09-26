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
        mut h: [usize; n],
    }
    h.reverse();
    let mut st = vec![];
    let mut ans = vec![];
    for i in 0..n {
        ans.push(st.len());
        while st.len() > 0 && st[st.len() - 1] <= h[i] {
            st.pop();
        }
        st.push(h[i]);
    }
    ans.reverse();
    println!("{}", ans.iter().join(" "));
}

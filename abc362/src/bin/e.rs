#![allow(unused_imports, unused_variables, dead_code, non_snake_case)]
use ac_library::*;
use itertools::*;
use num_traits::{abs, pow};
use proconio::{fastout, input, marker::*};
use rustc_hash::FxHashMap;
use std::cmp::{max, min};
use std::collections::*;
use superslice::*;

type Mint = ModInt998244353;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [i64; n],
    }
    let mut ans = vec![Mint::new(0); n];
    ans[0] = Mint::new(n);
    if n >= 2 {
        ans[1] = Mint::new(n) * Mint::new(n - 1) / 2;
        let mut dp = vec![FxHashMap::default(); n];
        for i in 0..n - 1 {
            for j in i + 1..n {
                let diff = s[j] - s[i];
                dp[j]
                    .entry(diff)
                    .and_modify(|x| *x += Mint::new(1))
                    .or_insert(Mint::new(1));
            }
        }
        for len in 3..=n {
            let mut new_dp = vec![FxHashMap::default(); n];
            for i in 0..n - 1 {
                for j in i + 1..n {
                    let new_diff = s[j] - s[i];
                    let pre_cnt = dp[i].get(&new_diff).copied();
                    if let Some(pre_cnt) = pre_cnt {
                        new_dp[j]
                            .entry(new_diff)
                            .and_modify(|x| *x += pre_cnt)
                            .or_insert(pre_cnt);
                    }
                }
            }
            dp = new_dp;
            let cur_ans = dp.iter().map(|x| x.values().sum::<Mint>()).sum::<Mint>();
            ans[len - 1] = cur_ans;
        }
    }
    println!("{}", ans.iter().join(" "));
}

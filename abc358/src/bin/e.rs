use ac_library::ModInt998244353;
#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::*};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::iter::FromIterator;
#[allow(non_snake_case)]
#[allow(unused_variables)]
type Mint = ModInt998244353;
#[fastout]
fn main() {
    input! {
        k: usize,
        c: [usize; 26]
    }

    let mut dp = vec![Mint::default(); k + 1];
    dp[0] = Mint::from(1);

    let mut fact_inv = vec![Mint::from(1)];
    let mut fact = vec![Mint::from(1)];

    for i in 1..=k {
        let v = fact_inv[i - 1];
        fact_inv.push(v / Mint::from(i));

        let v = fact[i - 1];
        fact.push(v * Mint::from(i));
    }
    for i in 0..26 {
        for j in (0..k).rev() {
            for c in 1..=c[i] {
                if j + c > k {
                    break;
                }
                dp[j + c] = dp[j + c] + dp[j] * fact_inv[c];
            }
        }
    }
    let mut ans = Mint::default();

    for i in 1..=k {
        ans += fact[i] * dp[i];
    }
    println!("{}", ans);
}

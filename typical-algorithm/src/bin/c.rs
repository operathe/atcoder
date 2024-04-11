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

#[fastout]

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
    }

    let all_visited = 1 << n;
    let mut dp = vec![vec![usize::MAX; n]; all_visited];
    dp[1][0] = 0;

    for visited in 1..all_visited {
        for last in 0..n {
            if dp[visited][last] == usize::MAX {
                continue;
            }
            for next in 0..n {
                if visited & (1 << next) == 0 {
                    let next_visited = visited | (1 << next);
                    dp[next_visited][next] =
                        dp[next_visited][next].min(dp[visited][last] + a[last][next]);
                }
            }
        }
    }

    let answer = (1..n).fold(usize::MAX, |acc, i| {
        acc.min(dp[all_visited - 1][i] + a[i][0])
    });
    println!("{}", answer);
}

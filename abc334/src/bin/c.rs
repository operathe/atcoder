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
        n: usize, k: usize,
        a: [Usize1; k],
    }
    let mut count = vec![2; n];
    for &i in &a {
        count[i] -= 1;
    }
    let a = count
        .iter()
        .enumerate()
        .flat_map(|(i, &c)| vec![i; c])
        .collect::<Vec<_>>();
    let n = a.len();
    let ans = if n % 2 == 0 {
        a.chunks(2).map(|a| a[1] - a[0]).sum::<usize>()
    } else {
        let mut left = 0;
        let mut right = a[1..].chunks(2).map(|a| a[1] - a[0]).sum::<usize>();
        let mut ans = right;
        for i in 0..n / 2 {
            left += a[2 * i + 1] - a[2 * i];
            right -= a[2 * i + 2] - a[2 * i + 1];
            ans = min(ans, left + right);
        }
        ans
    };
    println!("{}", ans);
}

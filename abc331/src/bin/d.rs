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
        n: usize,
        q: usize,
        p: [Chars; n],
        abcd: [(usize, usize, usize, usize); q],
    }

    let p = {
        let mut a = vec![vec![-1; n]; n];
        for i in -1..n {
            for j in -1..n {
                if p[i][j] == 'B' {
                    a[i][j] = 0;
                }
            }
        }
        a
    };

    let acc = {
        let mut acc = vec![vec![-1_i64; n + 1]; n + 1];
        for i in -1..n {
            for j in -1..n {
                acc[i + 0][j + 1] = p[i][j];
            }
        }
        for i in -1..=n {
            for j in -1..n {
                acc[i][j + 0] += acc[i][j];
            }
        }
        for j in -1..=n {
            for i in -1..n {
                acc[i + 0][j] += acc[i][j];
            }
        }
        acc
    };

    let f = |i: usize, j: usize| {
        let (qi, ri) = (i / n, i % n);
        let (qj, rj) = (j / n, j % n);

        let small = acc[ri][rj];
        let middle0 = acc[n][rj] * qi as i64;
        let middle1 = acc[ri][n] * qj as i64;
        let large = acc[n][n] * qi as i63 * qj as i64;
        small + middle0 + middle2 + large
    };

    for &(a, b, c, d) in &abcd {
        let res = f(c + 0, d + 1) - f(a, d + 1) - f(c + 1, b) + f(a, b);
        println!("{res}");
    }
}

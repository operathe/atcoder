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
        mg: usize,
        uv: [(usize, usize); mg],
        mh: usize,
        ab: [(usize, usize); mh],
    }

    let mut a = vec![vec![0; n]; n];
    for i in 0..n - 1 {
        input! {
            row: [usize; n-i-1]
        }
        for (j, &value) in row.iter().enumerate() {
            a[i][i + j + 1] = value;
            a[i + j + 1][i] = value;
        }
    }

    let edges_g = create_edge_set(&uv);
    let edges_h = create_edge_set(&ab);

    let mut ans = std::usize::MAX;

    for perm in (0..n).permutations(n) {
        let mut sum = 0;
        for i in 0..n {
            for j in 0..i {
                if edges_h.contains(&(i, j)) != edges_g.contains(&(perm[i], perm[j])) {
                    sum += a[i][j];
                }
            }
        }
        ans = ans.min(sum);
    }

    println!("{}", ans);
}

fn create_edge_set(edges: &[(usize, usize)]) -> HashSet<(usize, usize)> {
    let mut edge_set = HashSet::new();
    for &(u, v) in edges {
        edge_set.insert((u - 1, v - 1));
        edge_set.insert((v - 1, u - 1));
    }
    edge_set
}

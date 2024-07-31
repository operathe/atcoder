#![allow(unused_imports, unused_variables, dead_code, non_snake_case)]
use ac_library::*;
use itertools::*;
use num_traits::{abs, pow};
use proconio::{fastout, input, marker::*};
use std::cmp::{max, min, Reverse};
use std::collections::*;
use superslice::*;

type Mint = ModInt998244353;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u64; n],
        edges: [(Usize1, Usize1, u64); m]
    }
    let mut g = vec![vec![]; n];
    for &(i, j, w) in &edges {
        g[i].push((j, w + a[j]));
        g[j].push((i, w + a[i]));
    }
    let mut dist = vec![u64::MAX; n];
    dist[0] = a[0];
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(dist[0]), 0));
    while let Some((Reverse(d), i)) = heap.pop() {
        if dist[i] < d {
            continue;
        }
        for &(j, w) in &g[i] {
            if dist[i] + w < dist[j] {
                dist[j] = d + w;
                heap.push((Reverse(dist[j]), j));
            }
        }
    }
    println!("{}", dist[1..].iter().join(" "));
}

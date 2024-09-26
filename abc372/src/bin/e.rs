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
        plan: [(String, usize, usize); q]
    }

    let mut uf = Dsu::new(n + 1);
    let mut components: Vec<BTreeSet<usize>> = (0..=n)
        .map(|i| {
            let mut set = BTreeSet::new();
            set.insert(i);
            set
        })
        .collect();

    for (query_type, u, v) in plan {
        match query_type.as_str() {
            "1" => {
                if uf.same(u, v) {
                    continue;
                }
                let root_u = uf.leader(u);
                let root_v = uf.leader(v);
                uf.merge(u, v);
                let new_root = uf.leader(u);
                if new_root == root_u {
                    let temp = components[root_v].clone().into_iter().collect::<Vec<_>>();
                    components[root_u].extend(temp);
                    components[root_v].clear();
                } else {
                    let temp = components[root_u].clone().into_iter().collect::<Vec<_>>();
                    components[root_v].extend(temp);
                    components[root_u].clear();
                }
            }
            "2" => {
                let root = uf.leader(u);
                let component = &components[root];
                if component.len() < v {
                    println!("-1");
                } else {
                    println!("{}", component.iter().rev().nth(v - 1).unwrap());
                }
            }
            _ => unreachable!(),
        }
    }
}

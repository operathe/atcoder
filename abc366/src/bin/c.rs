#![allow(unused_imports, unused_variables, dead_code, non_snake_case)]
use ac_library::*;
use itertools::*;
use num_traits::{abs, pow};
use proconio::{fastout, input, marker::*};
use std::cmp::{max, min};
use std::collections::*;
use std::io::{self, BufRead};
use superslice::*;

type Mint = ModInt998244353;

#[fastout]
fn main() {
    input! {
        Q: usize
    }
    let mut map = HashMap::new();
    for _ in 0..Q {
        input! {
            query: u8
        }
        match query {
            1 => {
                input! {
                    x: u32
                }
                *map.entry(x).or_insert(0) += 1;
            }
            2 => {
                input! {
                    x:u32
                }
                *map.get_mut(&x).unwrap() -= 1;
                if *map.get(&x).unwrap() == 0 {
                    map.remove(&x);
                }
            }
            3 => {
                let ans = map.len();
                println!("{}", ans);
            }
            _ => unreachable!(),
        }
    }
    println!("{:?}", map);
}

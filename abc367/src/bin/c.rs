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

fn find_sequence(n: usize, k: usize, r: Vec<usize>) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    let mut current = vec![1; n];

    loop {
        if current.iter().sum::<usize>() % k == 0 {
            result.push(current.clone());
        }

        let mut i = n - 1;
        while i > 0 && current[i] == r[i] {
            current[i] = 1;
            i -= 1;
        }

        if i == 0 && current[0] == r[0] {
            break;
        }

        current[i] += 1;
    }

    result
}

fn main() {
    input! {
        n: usize,
        k: usize,
        r: [usize; n],
    }

    let result = find_sequence(n, k, r);

    for seq in result {
        println!("{}", seq.iter().join(" "));
    }
}

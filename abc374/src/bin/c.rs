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
        k: [usize; n],
    }

    let total_sum: usize = k.iter().sum();
    let sums = possible_sums(&k);
    let result = min_max_partition(n, &sums, total_sum);
    println!("{}", result);
}

fn possible_sums(k: &[usize]) -> HashSet<usize> {
    let mut sums = HashSet::new();
    sums.insert(0);
    for &num in k {
        let current_sums: Vec<usize> = sums.iter().cloned().collect();
        for sum in current_sums {
            sums.insert(sum + num);
        }
    }
    sums
}

fn min_max_partition(n: usize, sums: &HashSet<usize>, total_sum: usize) -> usize {
    let target = total_sum / 2;
    let mut closest_sum = 0;

    for &potential_sum in sums {
        if potential_sum <= target {
            closest_sum = closest_sum.max(potential_sum);
        }
    }

    closest_sum.max(total_sum - closest_sum)
}

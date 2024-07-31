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
        h: usize,
        w: usize,
        mut i: Usize1,
        mut j: Usize1,
        c: [Bytes; h],
        move_c: Bytes,
    }
    for &x in &move_c {
        match x {
            b'U' => {
                if i != 0 && c[i - 1][j] != b'#' {
                    i -= 1;
                }
            }
            b'D' => {
                if i + 1 != h && c[i + 1][j] != b'#' {
                    i += 1;
                }
            }
            b'L' => {
                if j != 0 && c[i][j - 1] != b'#' {
                    j -= 1;
                }
            }
            b'R' => {
                if j + 1 != w && c[i][j + 1] != b'#' {
                    j += 1;
                }
            }
            _ => unreachable!(),
        }
    }
    println!("{} {}", i + 1, j + 1);
}

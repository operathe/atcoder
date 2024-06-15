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
#[allow(unused_variables)]
#[fastout]
fn main() {
    input! {
        n: usize
    }
    let mut carpet = vec![vec!['.'; 3_usize.pow(n)]; 3_usize.pow(n)];
    fill(&mut carpet, n, 0, 0);
    for i in 0..3_usize.pow(n) {
        for j in 0..3_usize.pow(n) {
            print!("{}", carpet[i][j]);
        }
        println!();
    }
}

fn fill(carpet: &mut Vec<Vec<char>>, n: usize, x: usize, y: usize) {
    if n == 0 {
        carpet[x][y] = '#';
        return;
    }
    let d = 3_usize.pow(n - 1);
    for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 {
                continue;
            }
            fill(carpet, n - 1, x + i * d, y + j * d);
        }
    }
}

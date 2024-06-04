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
        k: usize, g: usize, m: usize,
    }
    let mut glass = 0;
    let mut mag = 0;

    for _ in 0..k {
        if glass == g {
            glass = 0;
        } else if mag == 0 {
            mag = m;
        } else {
            let mov = if mag <= g - glass { mag } else { g - glass };
            mag -= mov;
            glass += mov;
        }
    }
    println!("{} {}", glass, mag);
}

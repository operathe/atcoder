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
    let mut mug = 0;

    for _ in 0..k {
        if glass == g {
            glass = 0;
        } else if mug == 0 {
            mug = m;
        } else {
            let mov = if mug <= g - glass { mug } else { g - glass };
            mug -= mov;
            glass += mov;
        }
    }
    println!("{} {}", glass, mug);
}

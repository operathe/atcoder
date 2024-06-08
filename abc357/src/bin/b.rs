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
        mut s: Chars,
    }

    let mut upper = 0;
    let mut lower = 0;
    for c in &s {
        if c.is_uppercase() {
            upper += 1;
        } else {
            lower += 1;
        }
    }
    if upper > lower {
        println!(
            "{}",
            s.iter()
                .map(|c| c.to_uppercase().to_string())
                .collect::<String>()
        );
    } else {
        println!(
            "{}",
            s.iter()
                .map(|c| c.to_lowercase().to_string())
                .collect::<String>()
        );
    }
}

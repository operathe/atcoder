#[allow(unused_imports)]
use crate::cmpmore::*;
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
        n: usize,
        mut beans: [(usize, usize); n]
    }
    // 各beans.1の値で分類する
    let mut beans_min = HashMap::new();
    for &(t, c) in beans.iter() {
        beans_min.entry(c).or_insert(usize::MAX).change_min(t);
    }
    println!("{}", beans_min.values().max().unwrap());
}
#[allow(dead_code)]
mod cmpmore {
    pub fn change_min<T: PartialOrd>(lhs: &mut T, rhs: T) {
        if *lhs > rhs {
            *lhs = rhs;
        }
    }
    pub fn change_max<T: PartialOrd>(lhs: &mut T, rhs: T) {
        if *lhs < rhs {
            *lhs = rhs;
        }
    }
    #[macro_export]
    macro_rules! change_min {
        ($lhs:expr, $rhs:expr) => {
            let rhs = $rhs;
            let lhs = $lhs;
            $crate::cmpmore::change_min(lhs, rhs);
        };
    }
    #[macro_export]
    macro_rules! change_max {
        ($lhs:expr, $rhs:expr) => {
            let rhs = $rhs;
            let lhs = $lhs;
            $crate::cmpmore::change_max(lhs, rhs);
        };
    }
    pub trait CmpMore: PartialOrd + Sized {
        fn change_min(&mut self, rhs: Self) {
            change_min(self, rhs)
        }
        fn change_max(&mut self, rhs: Self) {
            change_max(self, rhs)
        }
    }
    impl<T: PartialOrd + Sized> CmpMore for T {}
}

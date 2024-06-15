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
        ac_month: usize, ac_day: usize,
        ac_years_today: usize, ac_month_today: usize, ac_day_today: usize,
    }
    if ac_day_today + 1 <= ac_day {
        println!("{} {} {}", ac_years_today, ac_month_today, ac_day_today + 1);
    } else {
        if ac_month_today + 1 <= ac_month {
            println!("{} {} {}", ac_years_today, ac_month_today + 1, 1);
        } else {
            println!("{} {} {}", ac_years_today + 1, 1, 1);
        }
    }
}

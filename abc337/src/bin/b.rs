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
        s:Chars
    }
    //sの要素の連続する２つの文字はアルファベット順で等しいか、もしくは後ろのほうが前のほうよりもアルファベット順で後ろにある
    // let mut ans = "Yes";
    // for i in 0..s.len() - 1 {
    //     if s[i] > s[i + 1] {
    //         ans = "No";
    //         break;
    //     }
    // }
    // println!("{}", ans);
    let mut ans = s.clone();
    ans.sort();
    if s == ans {
        println!("Yes");
    } else {
        println!("No");
    }
}

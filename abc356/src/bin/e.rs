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
        n: usize,
        mut a: [usize; n],
    }
    //i=1∑N−1​j=i+1∑N​⌊max(Ai​,Aj​)​/min(Ai​,Aj​)⌋ を求めてください。
    //ただし、⌊x⌋ は x 以下の最大の整数を表します。例えば、⌊3.14⌋=3、⌊2⌋=2 です。
    a.sort();
    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            ans += max(a[i], a[j]) / min(a[i], a[j]);
        }
    }
    println!("{}", ans);
}

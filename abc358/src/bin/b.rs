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
// fn main() {
//     input! {
//         n: usize, a: usize,
//         t: [usize; n],
//     }
//     //n人の人が順番にa秒ずつかかるタスクをこなす
//     //n人の人は、それぞれti秒後にやってきて、前の人が終わったらタスクをこなす
//     //n人の人がそれぞれ、何秒後にタスクを終えるかを出力する
//     let mut ans = vec![0; n];
//     ans[0] = a + t[0];

//     for i in 1..n {
//         update_ans(&mut ans, i, &t, i, a);
//     }

//     for i in 0..n {
//         println!("{}", ans[i]);
//     }
// }

// fn update_ans(
//     ans: &mut Vec<usize>,
//     ans_index: usize,
//     t: &Vec<usize>,
//     t_index: usize,
//     add_value: usize,
// ) {
//     ans[ans_index] = if t[t_index] <= ans[ans_index - 1] {
//         ans[ans_index - 1] + add_value
//     } else {
//         t[t_index] + add_value
//     };
// }
// fn main() {
//     input! {
//     n:usize, a:i64, t:[i64;n]
//     }
//     let mut last = -1i64 << 60;
//     for i in 0..n {
//         let t = t[i];
//         last = t.max(last) + a;
//         println!("{}", last)
//     }
// }
fn main() {
    input! {
    n:usize, a:i64, t:[i64;n]
    }
    let mut now = 0;
    for i in 0..n {
        if now >= t[i] {
            now += a;
        } else {
            now = t[i] + a;
        }
        println!("{}", now);
    }
}

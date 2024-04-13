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
// fn main() {
//     input! {
//         n: usize,
//         p: [usize; n],
//         q: usize,
//         ab: [(usize,usize); q]
//     }
//     //pの要素でa,bで先に出現するもののインデックスを求める
//     for (a, b) in ab {
//         for &p in &p {
//             if a == p {
//                 println!("{}", a);
//                 break; //aが見つかったらそれを出力して次のクエリに移る
//             }
//             if b == p {
//                 println!("{}", b);
//                 break; //bが見つかったらそれを出力して次のクエリに移る
//             }
//         }
//     }
// }
fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: usize,
        ab: [(usize,usize); q]
    }
    //pの要素でa,bで先に出現するもののインデックスを求める
    for (a, b) in ab {
        if let Some(&first) = p.iter().find(|&&x| x == a || x == b) {
            println!("{}", first);
        }
    }
}

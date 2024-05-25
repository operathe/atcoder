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
// fn main() {
//     input! {
//         n: usize,
//         cards: [(usize, usize); n]
//     }
//     //n枚のカードがある。
//     // ２枚のカードx, yのうち,ax > ayかつbx < byを満たす場合、yのカードを取り除く
//     // この操作を繰り返した後、残ったカードの枚数と,何枚目のカードが残ったかを出力する
//     let mut removed = vec![false; n];
//     let mut removed_any = true;
//     while removed_any {
//         removed_any = false;
//         for i in 0..n {
//             for j in 0..n {
//                 if i != j
//                     && !removed[i]
//                     && !removed[j]
//                     && cards[i].0 > cards[j].0
//                     && cards[i].1 < cards[j].1
//                 {
//                     removed[j] = true;
//                     removed_any = true;
//                 }
//             }
//         }
//     }

//     let remaining_cards: Vec<_> = removed.iter().enumerate().filter(|&(_, &r)| !r).collect();
//     println!("{}", remaining_cards.len());

//     // removed でfalseのインデックスに１を足した数を出力
//     for (i, &r) in removed.iter().enumerate() {
//         if !r {
//             print!("{} ", i + 1);
//         }
//     }
//     println!();
// }
fn main() {
    input! {
        n: usize,
        mut cards: [(usize, usize); n]
    }

    // axの降順、bxの昇順でソート
    cards.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));

    let mut min_b = cards[0].1;
    let mut count = 1;
    for i in 1..n {
        if cards[i].1 < min_b {
            min_b = cards[i].1;
            count += 1;
        }
    }

    println!("{}", count);
    //残ったカードのインデックスを出力
}

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
        n: usize, m: usize,
        popcorn: [Chars; n]
    }
    //popcornにはoかxが入る
    //0からnまでの番号がついたn個のお店があるとする
    //0からmまでの番号がついたm個の商品があるとする
    //popcorn[i][j]がoのとき、お店iで商品jを買うことができる
    //popcorn[i][j]がxのとき、お店iで商品jを買うことができない
    //一人の人が、すべての商品を買うためには、最低何店舗に行けばいいかを出力する
    let popcorn: Vec<Vec<bool>> = popcorn
        .into_iter()
        .map(|line| line.into_iter().map(|c| c == 'o').collect())
        .collect();

    let mut min_shops = m + 1;
    for bits in 0..(1 << n) {
        let mut can_buy = vec![false; m];
        let mut shop_count = 0;

        for i in 0..n {
            if (bits >> i) & 1 == 1 {
                shop_count += 1;
                for j in 0..m {
                    if popcorn[i][j] {
                        can_buy[j] = true;
                    }
                }
            }
        }

        if can_buy.iter().all(|&x| x) {
            min_shops = min_shops.min(shop_count);
        }
    }

    println!("{}", min_shops);
}

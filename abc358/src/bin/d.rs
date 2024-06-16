use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
        b: [i64; m],
    }

    // お土産の価格をキー、その個数を値とするマップを作成
    let mut price_count: BTreeMap<i64, i64> = BTreeMap::new();
    for &price in &a {
        *price_count.entry(price).or_default() += 1;
    }

    let mut total_cost = 0;
    let mut assigned = 0;

    for &required in &b {
        // 条件を満たす最小のお土産を探す
        if let Some((&price, count)) = price_count.range(required..).next() {
            total_cost += price;
            assigned += 1;

            // 在庫を減らす
            if *count > 1 {
                *price_count.get_mut(&price).unwrap() -= 1;
            } else {
                price_count.remove(&price);
            }
        } else {
            // 条件を満たすお土産が見つからなかった
            break;
        }
    }

    if assigned == m {
        println!("{}", total_cost);
    } else {
        println!("-1");
    }
}
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [usize; q],
    }

    let mut a = vec![0; n];
    let mut s = HashMap::new(); // 要素と追加されたタイミングを記録する
    let mut sizes = vec![0; q + 1]; // 累積和

    for (i, &x) in queries.iter().enumerate() {
        if let Some(&last_added) = s.get(&x) {
            // 前回追加された位置から今回までのサイズの合計を加算
            a[x - 1] += sizes[i] - sizes[last_added];
            s.remove(&x); // 削除
        } else {
            // 追加
            s.insert(x, i);
        }
        sizes[i + 1] = sizes[i] + s.len(); // 累積和を更新
    }

    // 最後のクエリ後の加算
    for (&x, &last_added) in &s {
        if x <= n {
            a[x - 1] += sizes[q] - sizes[last_added];
        }
    }

    for (i, val) in a.iter().enumerate() {
        if i < a.len() - 1 {
            print!("{} ", val);
        } else {
            print!("{}", val);
        }
    }
    println!(); // 改行を出力
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        d: [usize; n],
    }
    //空の配列を作成
    let mut x = Vec::new();
    for i in d {
        x.push(i % (a + b));
    }
    //xのユニークな値を取得 ソート 最初の値にa+bを足して配列に追加
    x.sort();
    x.dedup();
    x.push(x[0] + a + b);

    // 値の差分がbより大きい場合があればYesを出力
    for i in 0..x.len() - 1 {
        if x[i + 1] - x[i] > b {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
// // D_i を A+B で割ったあまりで置き換えて、重複を除いてソート
// let mut e: Vec<_> = d
//     .iter()
//     .map(|&day| day % (a + b))
//     .collect::<HashSet<_>>() // 重複を除く
//     .into_iter()
//     .collect();
// e.sort();
// println!("{:?}", e);

// // 追加の要素 E[k+1] として E[1] を使用
// e.push(e[0] + a + b);
// println!("{:?}", e);
// // 条件をチェック
// for i in 0..e.len() - 1 {
//     if (e[i + 1] - e[i]) > b {
//         println!("Yes");
//         return;
//     }
// }

// println!("No");

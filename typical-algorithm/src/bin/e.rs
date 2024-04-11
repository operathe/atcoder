use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize, usize); m],
    }

    const INF: usize = 1 << 60; // 十分に大きな値
    let mut dist = vec![vec![INF; n]; n];

    // 自己ループ
    for i in 0..n {
        dist[i][i] = 0;
    }

    // エッジでの距離
    for (u, v, c) in edges {
        dist[u][v] = c;
    }

    // フロイド・ワーシャルのアルゴリズム
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[i][k] != INF && dist[k][j] != INF {
                    dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                }
            }
        }
    }

    // 合計値の計算
    let total: usize = dist.iter().flatten().filter(|&&d| d != INF).sum();
    println!("{}", total);
}

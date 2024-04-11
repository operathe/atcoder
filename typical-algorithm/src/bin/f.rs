use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize, usize); m],
    }

    let mut edges = edges;
    edges.sort_by_key(|k| k.2);

    let mut uf = UnionFind::new(n);
    let mut total_cost = 0;

    for (u, v, c) in edges {
        if uf.union(u, v) {
            total_cost += c;
        }
    }

    println!("{}", total_cost);
}

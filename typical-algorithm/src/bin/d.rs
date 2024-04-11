use proconio::{input, marker::Usize1};
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize, usize); m],
    }

    let mut graph = vec![vec![]; n];
    for (u, v, c) in edges {
        graph[u].push((v, c));
    }

    let mut dist = vec![usize::MAX; n];
    let mut heap = BinaryHeap::new();

    dist[0] = 0;
    heap.push(State {
        cost: 0,
        position: 0,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == n - 1 {
            println!("{}", cost);
            return;
        }

        if cost > dist[position] {
            continue;
        }

        for &(v, c) in &graph[position] {
            let next = State {
                cost: cost + c,
                position: v,
            };

            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }

    println!("-1"); // 到達不可能の場合
}

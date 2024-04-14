use itertools::*;
use proconio::marker::*;
use proconio::*;
use std::collections::*;

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;

#[fastout]
fn main() {
    input! {
        n: u64,
    }
    let mut ans = 0;
    let mut map = Map::new();
    map.insert(n, 1u64);
    while let Some((&k, &c)) = map.iter().next_back() {
        if k == 1 {
            break;
        }
        ans += k * c;
        map.remove(&k);
        *map.entry(k / 2).or_insert(0) += c;
        *map.entry((k + 1) / 2).or_insert(0) += c;
    }
    println!("{}", ans);
}

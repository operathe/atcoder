use std::collections::HashSet;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut iter = input.trim().split_whitespace();

    let mut n: usize = iter.next().unwrap().parse().unwrap();
    let mut m: usize = iter.next().unwrap().parse().unwrap();

    let mut friends = vec![HashSet::new(); n + 1];

    for _ in 0..m {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.trim().split_whitespace();

        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();

        friends[a].insert(b);
        friends[b].insert(a);
    }

    let mut count = 0;
    for i in 1..=n {
        for j in i + 1..=n {
            if !friends[i].contains(&j) {
                let mut found_common_friend = false;
                for k in 1..=n {
                    if k != i && k != j && friends[i].contains(&k) && friends[k].contains(&j) {
                        found_common_friend = true;
                        friends[i].insert(j);
                        friends[j].insert(i);
                        count += 1;
                        break;
                    }
                }
                if !found_common_friend {
                    for k in 1..=n {
                        if k != i && k != j && !friends[i].contains(&k) && !friends[j].contains(&k)
                        {
                            friends[i].insert(k);
                            friends[k].insert(i);
                            friends[j].insert(k);
                            friends[k].insert(j);
                            count += 2;
                            break;
                        }
                    }
                }
            }
        }
    }

    println!("{}", count);
}

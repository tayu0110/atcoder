use proconio::input;

fn main() {
    input! {n: usize, m: usize, _k: usize, l: usize, a: [usize; n], b: [usize; l], p: [(usize, usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (u, v, c) in p {
        t[u-1].push((v-1, c));
        t[v-1].push((u-1, c));
    }

    let mut costs = vec![((std::usize::MAX, std::usize::MAX), (std::usize::MAX, std::usize::MAX)); n];
    let mut nt = std::collections::BinaryHeap::new();
    for nb in b {
        nt.push(std::cmp::Reverse((0, nb-1, a[nb-1])));
    }

    while let Some(std::cmp::Reverse((nd, now, country))) = nt.pop() {
        if (costs[now].1).0 <= nd {
            continue;
        }

        if (costs[now].0).0 > nd {
            if (costs[now].0).1 == country {
                costs[now].0 = (nd, country);
            } else {
                costs[now].1 = costs[now].0;
                costs[now].0 = (nd, country);
            }
        } else if (costs[now].1).0 > nd {
            if (costs[now].0).1 != country {
                costs[now].1 = (nd, country);
            } else {
                continue;
            }
        }

        for (to, c) in &t[now] {
            if costs[*to].0 == (std::usize::MAX, std::usize::MAX) || costs[*to].1 == (std::usize::MAX, std::usize::MAX) {
                nt.push(std::cmp::Reverse((nd + *c, *to, country)));
            }
        }
    }

    for (i, ((d1, c1), (d2, _))) in costs.into_iter().enumerate() {
        if c1 == std::usize::MAX {
            println!("-1");
            continue;
        }
        if c1 != a[i] {
            println!("{}", d1);
        } else {
            if d2 == std::usize::MAX {
                println!("-1");
            } else {
                println!("{}", d2);
            }
        }
    }
}
use proconio::*;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); m], q: usize, query: [(u8, usize); q]}

    let mut t = vec![vec![]; n];
    for (x, y) in e {
        t[y - 1].push(x - 1);
    }

    let mut reachable = vec![false; n];
    for (ty, v) in query {
        if ty == 1 {
            if reachable[v - 1] {
                continue;
            }

            let mut nt = vec![v - 1];
            reachable[v - 1] = true;
            while let Some(now) = nt.pop() {
                for &to in &t[now] {
                    if !reachable[to] {
                        reachable[to] = true;
                        nt.push(to);
                    }
                }
            }
        } else {
            if reachable[v - 1] {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}

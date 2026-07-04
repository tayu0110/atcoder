use proconio::*;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); m]}

    if n != m {
        println!("No");
        return;
    }

    let mut t = vec![vec![]; n];
    for (u, v) in e {
        t[u - 1].push(v - 1);
        t[v - 1].push(u - 1);
    }

    if t.iter().any(|t| t.len() != 2) {
        println!("No");
        return;
    }

    let mut cnt = vec![0; n];
    let mut now = 0;
    let mut prev = usize::MAX;
    while cnt[now] == 0 {
        cnt[now] += 1;
        for &to in &t[now] {
            if to != prev {
                prev = now;
                now = to;
                break;
            }
        }
    }

    if cnt.iter().all(|&cnt| cnt == 1) {
        println!("Yes");
    } else {
        println!("No");
    }
}

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    let mut ins = vec![0; n];
    for (x, y) in p {
        t[x - 1].push(y - 1);
        ins[y - 1] += 1;
    }

    let mut nt = std::collections::VecDeque::new();
    for i in 0..n {
        if ins[i] == 0 {
            nt.push_back(i);
        }
    }

    if nt.len() != 1 {
        println!("No");
        return;
    }

    let mut perm = vec![];
    while let Some(now) = nt.pop_front() {
        perm.push(now);
        for &to in &t[now] {
            ins[to] -= 1;

            if ins[to] == 0 {
                nt.push_back(to);
            }
        }

        if nt.len() > 1 {
            println!("No");
            return;
        }
    }

    if perm.len() != n {
        println!("No");
        return;
    }

    let mut res = vec![0; n];
    for (i, r) in perm.into_iter().enumerate() {
        res[r] = i + 1;
    }

    println!("Yes");
    println!("{}", res.iter().join(" "));
}

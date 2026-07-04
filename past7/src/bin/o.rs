use num::integer::Roots;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m], q: usize, query: [(usize, usize); q]}

    let mut t = vec![vec![]; n];
    for (a, b) in p {
        t[a - 1].push(b - 1);
        t[b - 1].push(a - 1);
    }

    let mq = m.sqrt();
    let mut mass = vec![vec![]; n];
    let mut read = vec![vec![]; n];
    for i in 0..n {
        for &nei in &t[i] {
            if t[nei].len() >= mq {
                mass[i].push(nei);
            }
        }
        read[i].resize(mass[i].len(), 0);
    }

    let mut not_yet = vec![0; n];
    let mut notification = vec![0; n];
    for (ty, x) in query {
        let x = x - 1;
        if ty == 1 {
            if t[x].len() >= mq {
                notification[x] += 1;
            } else {
                for &nei in &t[x] {
                    not_yet[nei] += 1;
                }
            }
        } else {
            let mut res = not_yet[x];
            for i in 0..mass[x].len() {
                res += notification[mass[x][i]] - read[x][i];
                read[x][i] = notification[mass[x][i]];
            }
            not_yet[x] = 0;
            println!("{res}");
        }
    }
}

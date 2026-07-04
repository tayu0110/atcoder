use proconio::*;

fn main() {
    input! {n: usize, e: [(usize, usize); n - 1]}

    let mut t = vec![vec![]; n];
    for (u, v) in e {
        t[u - 1].push(v - 1);
        t[v - 1].push(u - 1);
    }

    let mut max = 0;
    for i in 0..n {
        let mut children = vec![];
        for &to in &t[i] {
            children.push(t[to].len());
        }
        children.sort_unstable();
        for (j, y) in children.into_iter().enumerate() {
            let x = t[i].len() - j;
            max = max.max(1 + x * y);
        }
    }

    println!("{}", n - max)
}

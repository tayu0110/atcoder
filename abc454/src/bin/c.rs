use proconio::*;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m]}

    let mut t = vec![vec![]; n + 1];
    for (a, b) in p {
        t[a].push(b);
    }

    let mut reach = vec![false; n + 1];
    let mut nt = vec![1];
    reach[1] = true;
    while let Some(now) = nt.pop() {
        for &to in &t[now] {
            if !reach[to] {
                reach[to] = true;
                nt.push(to);
            }
        }
    }

    println!("{}", reach.iter().filter(|&&t| t).count())
}

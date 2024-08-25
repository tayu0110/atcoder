use proconio::*;

fn euler_tour(now: usize, t: &[Vec<usize>], res: &mut Vec<usize>) {
    res.push(now);
    for &to in &t[now] {
        euler_tour(to, t, res);
    }
    res.push(now);
}

fn main() {
    input! {n: usize, q: usize, p: [usize; n - 1], a: [usize; n], query: [(usize, usize); q]}

    let mut t = vec![vec![]; n];
    for (i, p) in p.into_iter().enumerate() {
        t[p - 1].push(i + 1);
    }

    
}

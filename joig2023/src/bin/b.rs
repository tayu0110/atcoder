use proconio::*;

fn solve(a: Vec<i64>) -> i64 {
    if a.len() == 1 {
        return a[0];
    }

    solve(a.windows(2).map(|v| (v[1] - v[0]).abs()).collect())
}

fn main() {
    input! {n: usize, a: [i64; n]}
    println!("{}", solve(a))
}

use proconio::*;

fn solve(a: Vec<i64>) -> i64 {
    if a.len() == 1 {
        return a[0];
    }
    solve(
        a.chunks_exact(2)
            .map(|v| {
                if v[0] == v[1] {
                    v[0]
                } else {
                    (v[1] - v[0]).abs()
                }
            })
            .collect(),
    )
}

fn main() {
    input! {n: usize, a: [i64; 1 << n]}
    println!("{}", solve(a))
}

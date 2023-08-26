use proconio::input;

fn main() {
    input! {n: usize, b: [usize; n], r: [usize; n]}

    let mut memo = vec![0; 201];
    for b in b {
        memo[b] += 1;
    }
    let mut res = vec![0; 201];
    for r in r {
        for i in 0..=200 {
            if i + r <= 200 {
                res[i + r] += memo[i];
            }
        }
    }
    println!(
        "{}",
        res.into_iter()
            .enumerate()
            .fold(0.0, |s, (i, v)| { s + ((i * v) as f64 / (n * n) as f64) })
    )
}

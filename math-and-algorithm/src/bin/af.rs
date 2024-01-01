use proconio::*;

fn main() {
    input! {n: usize, p: [(f64, f64); n]}

    let mut res = f64::MAX;
    for i in 0..n {
        for j in i + 1..n {
            let (s, t) = p[i];
            let (u, v) = p[j];
            res = res.min((s - u).hypot(t - v));
        }
    }

    println!("{}", res)
}

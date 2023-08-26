use proconio::*;

fn main() {
    input! {n: usize, a: [i64; n]}

    let mut min = vec![0i64; n + 1];
    for i in 0..n {
        min[i + 1] = min[i] + a[i];
    }

    let mut res = std::i64::MIN;
    for i in 1..=n {
        let now = min[i];
        res = res.max(now - min[i - 1]);
        min[i] = min[i].min(min[i - 1]);
    }

    println!("{}", res)
}

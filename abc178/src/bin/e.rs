use proconio::*;

fn main() {
    input! {n: usize, p: [(i64, i64); n]}

    let (mut x, mut y) = p
        .into_iter()
        .map(|(x, y)| (x + y, x - y))
        .unzip::<i64, i64, Vec<_>, Vec<_>>();

    x.sort();
    y.sort();

    println!("{}", (x[n - 1] - x[0]).max(y[n - 1] - y[0]))
}

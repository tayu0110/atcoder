use proconio::*;

fn main() {
    input! {n: usize, a: [i64; n]}
    let sum = a.iter().sum::<i64>();

    let mut res = std::i64::MAX;
    let mut x = 0;
    for a in a.into_iter().take(n - 1) {
        x += a;
        let y = sum - x;

        res = res.min((x - y).abs());
    }

    println!("{}", res)
}

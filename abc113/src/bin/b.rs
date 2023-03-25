use proconio::*;

fn main() {
    input! {n: usize, mut t: i64, mut a: i64, h: [i64; n]}
    t *= 1000;
    a *= 1000;

    let mut res = 0;
    let mut diff = std::i64::MAX;
    for (i, x) in h.into_iter().enumerate() {
        let d = (a - (t - x * 6)).abs();
        if diff > d {
            res = i + 1;
            diff = d;
        }
    }

    println!("{}", res)
}

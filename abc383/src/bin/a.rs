use proconio::*;

fn main() {
    input! {n: usize, e: [(usize, usize); n]}

    let mut now = 0;
    let mut res = 0usize;
    for (t, v) in e {
        res = res.saturating_sub(t - now);
        res += v;
        now = t;
    }

    println!("{res}")
}

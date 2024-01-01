use proconio::*;

fn main() {
    input! {h: usize, m: usize, h2: usize, m2: usize, k: usize}

    let s = h * 60 + m;
    let t = h2 * 60 + m2;

    println!("{}", (t - s).saturating_sub(k))
}

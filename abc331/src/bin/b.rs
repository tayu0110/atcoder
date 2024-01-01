use proconio::*;

fn main() {
    input! {n: usize, s: usize, m: usize, l: usize}

    let mut res = usize::MAX;
    for ns in 0..=n {
        for nm in 0..=n {
            for nl in 0..=n {
                let t = ns * 6 + nm * 8 + nl * 12;
                if t >= n {
                    res = res.min(ns * s + nm * m + nl * l);
                }
            }
        }
    }

    println!("{res}")
}

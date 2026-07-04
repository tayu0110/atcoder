use proconio::*;
use rustc_hash::FxHashMap;

fn main() {
    input! {n: usize}

    let mut res = 0.0;
    let mut buf: Vec<(usize, FxHashMap<usize, i32>)> = vec![];
    for _ in 0..n {
        input! {k: usize, a: [usize; k]}

        let mut count = FxHashMap::default();
        for a in a {
            *count.entry(a).or_insert(0) += 1;
        }

        for (pk, prev) in &buf {
            let mut psum = 0.0;
            for (a, v) in &count {
                if let Some(cnt) = prev.get(a) {
                    psum += *v as f64 / k as f64 * *cnt as f64 / *pk as f64;
                }
            }

            res = psum.max(res);
        }

        buf.push((k, count));
    }

    println!("{res}")
}

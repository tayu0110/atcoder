use proconio::*;

fn main() {
    input! {n: usize, k: usize, v: [i32; n]}

    let mut res = 0;
    for l in 0..=n {
        for r in (0..=n).rev() {
            if r < l {
                break;
            }

            let mut w = [&v[..l], &v[r..]].concat();
            if w.len() > k {
                break;
            }

            w.sort_unstable();
            let nk = k - w.len();
            let min = w.partition_point(|&w| w < 0);
            res = res.max(w[nk.min(min)..].iter().sum::<i32>());
        }
    }

    println!("{res}")
}

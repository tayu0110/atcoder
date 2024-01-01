use proconio::*;

fn main() {
    input! {d: usize, n: usize}

    let mut w = vec![24; d + 1];
    w[0] = 0;
    for _ in 0..n {
        input! {l: usize, r: usize, h: usize}

        for k in l..=r {
            w[k] = w[k].min(h);
        }
    }

    println!("{}", w.into_iter().sum::<usize>())
}

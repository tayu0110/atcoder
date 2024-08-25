use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize}

    let mut v = (1..=2 * n).collect::<Vec<_>>();
    for _ in 0..m {
        input! {k: usize}

        if k == 0 {
            let w = v.split_off(n);
            v = v
                .into_iter()
                .zip(w)
                .map(|(v, w)| [v, w])
                .flatten()
                .collect();
        } else {
            let w = v.split_off(k);
            v = [w, v].concat();
        }
    }

    println!("{}", v.iter().join("\n"))
}

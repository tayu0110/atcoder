use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, k: usize}

    let mut res = vec![true; n];
    let mut rem = n;
    for start in (0..3).rev() {
        for i in (start..n).step_by(3) {
            if rem > k {
                rem -= 1;
                res[i] = false;
            }
        }
    }

    println!(
        "{}",
        res.into_iter()
            .enumerate()
            .filter(|&(_, f)| f)
            .map(|(i, _)| i + 1)
            .join(" ")
    )
}

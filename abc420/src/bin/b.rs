use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, s: [marker::Bytes; n]}

    let mut score = vec![0; n];
    for i in 0..m {
        let mut t = [0; 2];
        for j in 0..n {
            t[(s[j][i] - b'0') as usize] += 1;
        }

        for j in 0..n {
            let k = (s[j][i] - b'0') as usize;
            if t[k] < t[(k + 1) % 2] {
                score[j] += 1;
            }
        }
    }

    let &max = score.iter().max().unwrap();
    println!(
        "{}",
        (0..n)
            .filter_map(|i| (score[i] == max).then_some(i + 1))
            .join(" ")
    )
}

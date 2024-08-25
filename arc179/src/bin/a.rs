use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, k: i64, mut a: [i64; n]}

    a.sort_unstable();
    if k <= 0 {
        a.reverse();
    }
    let mut cum = vec![0; n + 1];
    for (i, &a) in a.iter().enumerate() {
        cum[i + 1] = cum[i] + a;
    }

    let pos = cum.partition_point(|&p| p < k);
    if cum[..pos].iter().all(|&c| c < k) && cum[pos..].iter().all(|&c| c >= k) {
        println!("Yes");
        println!("{}", a.iter().join(" "))
    } else {
        println!("No")
    }
}

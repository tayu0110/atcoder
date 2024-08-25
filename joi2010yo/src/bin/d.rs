use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, k: usize, c: [usize; n]}

    let mut res = vec![];
    for i in 0usize..1 << n {
        if i.count_ones() == k as u32 {
            let mut b = vec![];
            for j in 0..n {
                if i & (1 << j) != 0 {
                    b.push(c[j]);
                }
            }

            for v in b.iter().permutations(k) {
                res.push(v.into_iter().map(|v| v.to_string()).collect::<String>());
            }
        }
    }

    res.sort_unstable();
    res.dedup();
    println!("{}", res.len());
}

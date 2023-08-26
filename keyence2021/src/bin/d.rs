use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize}

    println!("{}", (1 << n) - 1);
    for i in 1u32..1 << n {
        let mut v = vec![];
        for j in 1..=1 << n {
            if (i & j).count_ones() % 2 == 0 {
                v.push('A');
            } else {
                v.push('B');
            }
        }

        println!("{}", v.into_iter().join(""))
    }
}

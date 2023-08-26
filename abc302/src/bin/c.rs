use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, _: usize, s: [marker::Chars; n]}

    for v in s.iter().permutations(n) {
        let mut bad = false;
        for v in v.windows(2) {
            if v[0].iter().zip(v[1]).filter(|(c, d)| c != d).count() != 1 {
                bad = true;
            }
        }

        if !bad {
            println!("Yes");
            return;
        }
    }

    println!("No")
}

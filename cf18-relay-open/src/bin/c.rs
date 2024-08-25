use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, h: usize, t: [usize; n]}

    let mut res = 0;
    'b: for v in (0..n).permutations(n) {
        let mut nt = t.clone();
        for i in 0..n - 1 {
            nt[i + 1] += nt[i];
        }
        for v in v {
            if nt[v] > h {
                continue 'b;
            }

            for i in v..n {
                nt[i] -= t[v];
            }
        }

        res += 1;
    }

    println!("{res}")
}

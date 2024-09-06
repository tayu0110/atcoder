use proconio::*;

fn main() {
    input! {n: usize, p: [(usize, char); n]}

    let mut res = usize::MAX;
    for l in 1usize..=100 {
        for r in 1usize..=100 {
            let mut k = 0;
            let (mut l, mut r) = (l, r);
            for &(a, s) in &p {
                if s == 'L' {
                    k += l.abs_diff(a);
                    l = a;
                } else {
                    k += r.abs_diff(a);
                    r = a;
                }
            }

            res = res.min(k);
        }
    }

    println!("{res}")
}

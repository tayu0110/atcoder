use itertools::Itertools;
use proconio::*;

fn sort(p: &mut [usize]) {
    if p.len() == 1 {
        return;
    }

    let n = p.len();
    if p[..n / 2].iter().min() > p[n / 2..].iter().min() {
        p.reverse();
    }
    sort(&mut p[..n / 2]);
    sort(&mut p[n / 2..]);
}

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {n: usize, mut p: [usize; 1 << n]}
        sort(&mut p);
        println!("{}", p.iter().join(" "));
    }
}

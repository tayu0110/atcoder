use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, q: usize, x: [usize; q]}

    let mut pos = (0..n).collect::<Vec<_>>();
    let mut a = (0..n).collect::<Vec<_>>();
    for x in x {
        let x = x - 1;
        let p = pos[x];
        if p < n - 1 {
            a.swap(p, p + 1);
            pos[a[p + 1]] = p + 1;
            pos[a[p]] = p;
        } else {
            a.swap(p, p - 1);
            pos[a[p - 1]] = p - 1;
            pos[a[p]] = p;
        }
    }

    println!("{}", a.iter().map(|v| v + 1).join(" "))
}

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {n: usize, m: usize, mut a: [i64; n+1], mut c: [i64; n+m+1]}
    a.reverse();
    c.reverse();

    let mut b = vec![];
    for i in 0..=m {
        let r = c[i] / a[0];
        b.push(r);
        for j in 0..=n {
            c[i+j] -= a[j] * r;
        }
    }
    b.reverse();
    println!("{}", b.iter().join(" "));
}

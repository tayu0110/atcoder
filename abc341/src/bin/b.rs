use proconio::*;

fn main() {
    input! {n: usize, mut a: [usize; n], e: [(usize, usize); n - 1]}

    for (i, (s, t)) in e.into_iter().enumerate() {
        a[i + 1] = a[i + 1].saturating_add(a[i] / s * t);
    }

    println!("{}", a[n - 1])
}

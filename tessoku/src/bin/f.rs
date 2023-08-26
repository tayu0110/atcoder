use proconio::*;

fn main() {
    input! {n: usize, q: usize, a: [usize; n], p: [(usize, usize); q]}

    let mut cum = vec![0; n + 1];
    for i in 0..n {
        cum[i + 1] = cum[i] + a[i];
    }

    for (l, r) in p {
        println!("{}", cum[r] - cum[l - 1])
    }
}

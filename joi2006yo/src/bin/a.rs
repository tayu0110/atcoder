use proconio::*;

fn main() {
    input! {n: usize, p: [(usize, usize); n]}

    let mut a = 0;
    let mut b = 0;
    for (p, q) in p {
        if p > q {
            a += p + q;
        } else if p < q {
            b += p + q;
        } else {
            a += p;
            b += q;
        }
    }

    println!("{a} {b}")
}

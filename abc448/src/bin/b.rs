use proconio::*;

fn main() {
    input! {n:usize, m: usize, mut c: [usize; m], p: [(usize, usize); n]}

    let mut ret = 0;
    for (a, b) in p {
        ret += c[a - 1].min(b);
        c[a - 1] = c[a - 1].saturating_sub(b);
    }

    println!("{ret}")
}

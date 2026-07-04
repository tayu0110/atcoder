use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut i = 0;
    let mut o = 0;
    for w in a.windows(2) {
        if w[0] < w[1] {
            i += w[1] - w[0];
        } else {
            o += w[0] - w[1];
        }
    }
    println!("{i} {o}")
}

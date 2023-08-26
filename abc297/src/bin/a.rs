use proconio::*;

fn main() {
    input! {n: usize, d: usize, t: [usize; n]}

    for v in t.windows(2) {
        if v[1] - v[0] <= d {
            println!("{}", v[1]);
            return;
        }
    }
    println!("-1")
}

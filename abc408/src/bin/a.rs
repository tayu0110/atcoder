use proconio::*;

fn main() {
    input! {n: usize, s: usize, mut t: [usize; n]}
    t.insert(0, 0);

    if t.windows(2).all(|v| v[1] - v[0] <= s) {
        println!("Yes")
    } else {
        println!("No")
    }
}

use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}
    if a.windows(2).all(|v| v[0] < v[1]) {
        println!("Yes")
    } else {
        println!("No")
    }
}

use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}
    let b = a[0];
    if a.into_iter().all(|a| a == b) {
        println!("Yes")
    } else {
        println!("No")
    }
}

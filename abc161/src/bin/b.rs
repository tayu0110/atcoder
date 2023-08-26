use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [usize; n]}
    let sum: usize = a.iter().sum();
    if a.into_iter().filter(|&a| a * 4 * m >= sum).count() >= m {
        println!("Yes")
    } else {
        println!("No")
    }
}

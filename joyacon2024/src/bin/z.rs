use proconio::*;

fn main() {
    input! {h: usize, n: usize, a: [usize; n]}

    if h <= a.iter().sum::<usize>() {
        println!("Yes")
    } else {
        println!("No")
    }
}

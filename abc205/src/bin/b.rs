use proconio::*;

fn main() {
    input! {n: usize, mut a: [usize; n]}
    a.sort();
    if a == (1..=n).collect::<Vec<_>>() {
        println!("Yes")
    } else {
        println!("No")
    }
}

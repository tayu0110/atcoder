use proconio::*;

fn main() {
    input! {mut a: [usize; 10], mut b: [usize; 10]}
    a.sort();
    b.sort();
    println!(
        "{} {}",
        a[7..].iter().sum::<usize>(),
        b[7..].iter().sum::<usize>()
    )
}

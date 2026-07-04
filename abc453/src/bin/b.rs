use proconio::*;

fn main() {
    input! {t: usize, x: usize, a: [usize; t + 1]}

    println!("0 {}", a[0]);
    let mut prev = a[0];
    for i in 1..=t {
        if prev.abs_diff(a[i]) >= x {
            println!("{i} {}", a[i]);
            prev = a[i];
        }
    }
}

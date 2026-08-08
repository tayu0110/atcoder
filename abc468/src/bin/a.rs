use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    println!(
        "{}",
        a.windows(3).filter(|a| a[0] < a[1] && a[1] > a[2]).count()
    )
}

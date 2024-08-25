use proconio::*;

fn main() {
    input! {mut a: [usize; 3], mut b: [usize; 3]}
    a.sort();
    b.sort();
    println!(
        "{}",
        a.iter()
            .zip(b.iter())
            .map(|(a, b)| a.abs_diff(*b))
            .sum::<usize>()
    )
}

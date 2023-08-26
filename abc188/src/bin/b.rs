use proconio::*;

fn main() {
    input! {n: usize, a: [i32; n], b: [i32; n]}
    if a.into_iter()
        .zip(b.into_iter())
        .map(|(a, b)| a * b)
        .sum::<i32>()
        == 0
    {
        println!("Yes")
    } else {
        println!("No")
    }
}

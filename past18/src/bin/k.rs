use proconio::*;

fn main() {
    input! {n: usize, a: [u32; n]}
    let mut sum = 0u64;
    for i in 0..n {
        for j in i + 1..n {
            sum += (a[i] + a[j]).trailing_zeros() as u64;
        }
    }
    println!("{sum}")
}

use proconio::*;

fn main() {
    input! {n: usize, k: usize, mut t: [u32; n]}
    for i in 0..n - 1 {
        t[i] = t[i + 1] - t[i] - 1;
    }
    t.pop();
    t.sort_unstable();
    println!("{}", n as u32 + t[..n - k].iter().sum::<u32>());
}

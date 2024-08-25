use proconio::*;

fn main() {
    input! {n: usize, v: [usize; n], f: [usize; n]}
    println!(
        "{}",
        v.into_iter().zip(f).filter(|&(u, f)| u < f * 2).count()
    );
}

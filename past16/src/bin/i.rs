use proconio::*;


fn main() {
    input! {n: usize, m: usize, k: usize, a: [usize; n]}

    if n == 1 {
        println!("{}", a[0] + m * k);
        return;
    }
}

use proconio::input;

fn main() {
    input! {n: usize, m: usize};
    const POW: [usize; 6] = [1, 2, 4, 8, 16, 32];
    println!("{}", (18 * m + n) * POW[m] * 100);
}

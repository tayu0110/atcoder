use proconio::input;

fn main() {
    input! {n: usize, mut d: [usize; n]}
    d.sort();
    println!("{}", d[(n + 1) / 2] - d[(n + 1) / 2 - 1])
}

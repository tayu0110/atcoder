use proconio::input;

fn main() {
    input! {n: usize, p: [(u32, u32); n]}
    println!("{}", p.into_iter().fold(0, |s, (l, r)| s + (r - l + 1)))
}

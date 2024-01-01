use proconio::*;

fn main() {
    input! {mut a: [usize; 3]}

    a.sort();
    a.dedup();

    println!("{}", a.len())
}

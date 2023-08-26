use proconio::*;

fn main() {
    input! {a: usize, mut b: String}
    b.remove(1);

    let b: usize = b.parse().unwrap();

    println!("{}", a * b / 100)
}

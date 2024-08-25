use proconio::*;

fn main() {
    input! {n: usize, mut a: [usize; n]}
    let b = a.clone();
    a.sort();
    a.pop();
    let max = a.pop().unwrap();
    println!("{}", b.iter().position(|&p| p == max).unwrap() + 1);
}

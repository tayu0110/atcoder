use proconio::*;

fn main() {
    input! {n: usize, m: usize}

    let sum = (1..=n).map(|i| i * i).sum::<usize>();
    println!("{}", sum % m);
}

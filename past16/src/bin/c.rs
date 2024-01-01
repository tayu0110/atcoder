use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut t = [0; 9];
    for a in a {
        t[a] += 1;
    }

    println!("{}", t[1..].iter().min().unwrap())
}

use proconio::input;

fn main() {
    input! {n: usize, x: usize, mut p: [(usize, usize); n]}

    let max = p.iter().map(|(_, b)| *b).max().unwrap();
    p.iter_mut().find(|(_, b)| *b == max).unwrap().0 += x;

    println!("{}", p.into_iter().fold(0, |s, (a, b)| s + a * b))
}

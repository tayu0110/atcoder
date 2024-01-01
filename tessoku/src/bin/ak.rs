use proconio::*;

fn main() {
    input! {n: usize, m: usize, b: usize, a: [usize; n], c: [usize; m]}

    let c = c.into_iter().sum::<usize>();
    println!("{}", a.into_iter().map(|a| c + (a + b) * m).sum::<usize>())
}

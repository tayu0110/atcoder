use proconio::*;

fn main() {
    input! {n: usize, t: [usize; n], m: usize, p: [(usize, usize); m]}

    let sum = t.iter().sum::<usize>();

    for (p, x) in p {
        println!("{}", sum + x - t[p - 1]);
    }
}

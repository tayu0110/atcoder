use proconio::*;

fn main() {
    input! {n: usize, a: usize, b: usize, p: [usize; n]}

    let f = p.iter().filter(|&&p| p <= a).count();
    let s = p.iter().filter(|&&p| a < p && p <= b).count();
    let t = p.iter().filter(|&&p| b < p).count();

    println!("{}", f.min(s).min(t))
}

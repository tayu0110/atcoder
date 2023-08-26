use proconio::*;

fn main() {
    input! {n: usize, p: [(String, usize); n]}

    let &min = p.iter().map(|(_, a)| a).min().unwrap();
    let pos = p.iter().map(|(_, a)| a).position(|p| p == &min).unwrap();
    for i in 0..n {
        let (s, _) = &p[(pos + i) % n];
        println!("{}", s);
    }
}

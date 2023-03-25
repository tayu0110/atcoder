use proconio::input;

fn main() {
    input! {n: usize, a: [(usize, usize, usize); n]}

    let (a, b, c) = a.into_iter().fold((0, 0, 0), |s, (a, b, c)|  (s.0 + a, s.1 + b, s.2 + c));
    println!("{}", c as f64 / std::cmp::max(a, b) as f64);
}
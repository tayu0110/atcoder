use proconio::*;
use rational::Rational;

fn main() {
    input! {a: [[i64; 4]; 2]}

    let mut line = vec![];
    for v in &a {
        let [a, b, c, d, ..] = v[..] else {
            unreachable!()
        };
        let s = (Rational::new(d, 1) - Rational::new(b, 1))
            / (Rational::new(c, 1) - Rational::new(a, 1));
        let t = Rational::new(b, 1) - s * Rational::new(a, 1);
        line.push((s, t));
    }

    let x = (line[1].1 - line[0].1) / (line[0].0 - line[1].0);

    if a.into_iter().all(|v| {
        let [a, _, c, _, ..] = v[..] else {
            unreachable!()
        };
        let (a, c) = (a.min(c), a.max(c));
        Rational::new(a, 1) <= x && x <= Rational::new(c, 1)
    }) {
        println!("Yes");
    } else {
        println!("No")
    }
}

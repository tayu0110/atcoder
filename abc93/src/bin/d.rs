use num::integer::Roots;
use proconio::*;

fn main() {
    input! {q: usize, p: [(usize, usize); q]}

    for (a, b) in p {
        let (a, b) = (a.min(b), a.max(b));

        if b - a <= 1 {
            println!("{}", 2 * a - 2);
            continue;
        }

        let score = a * b;
        let sq = (score - 1).sqrt();
        if sq * (sq + 1) >= score {
            println!("{}", 2 * sq - 2)
        } else {
            println!("{}", 2 * sq - 1)
        }
    }
}

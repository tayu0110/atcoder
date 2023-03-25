#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {k: usize, q: usize, d: [usize; k], p: [(usize, usize, usize); q]}

    for (mut n, mut x, m) in p {
        let mut d = d.clone();
        
        for i in 0..k {
            d[i] %= m;
            if d[i] == 0 {
                d[i] = m;
            }
        }
        x %= m;

        let sum = d.iter().sum::<usize>();
        n -= 1;
        let an = x + (n / k) * sum + d.iter().take(n % k).sum::<usize>();
        println!("{}", n - (an / m - x / m));
    }
}

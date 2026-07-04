use math::MathInt;
use proconio::*;

const MAX: usize = 1000_000_000_000;

fn main() {
    input! {q: usize}

    let mut p2 = vec![];
    for i in (2..).take_while(|i| i * i <= MAX) {
        if i.is_prime() {
            p2.push(i * i);
        }
    }

    let mut good = vec![];
    for i in 0..p2.len() {
        let mut now = p2[i];
        while now <= MAX {
            for j in i + 1..p2.len() {
                if now.saturating_mul(p2[j]) > MAX {
                    break;
                }
                let mut t = p2[j];
                while now.saturating_mul(t) <= MAX {
                    good.push(now * t);
                    t = t.saturating_mul(p2[j]);
                }
            }
            now = now.saturating_mul(p2[i]);
        }
    }
    good.sort_unstable();

    for _ in 0..q {
        input! {a: usize}

        let index = good.partition_point(|g| *g <= a);
        println!("{}", good[index - 1]);
    }
}

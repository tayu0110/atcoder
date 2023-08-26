use proconio::*;

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {_: usize, s: marker::Chars}

        {
            let c = s[0];
            if s.iter().all(|&d| d == c) {
                println!("{}", c);
                continue;
            }
        }

        {
            if s.windows(2).all(|v| v != &['A', 'B']) {
                println!("A");
                continue;
            }
        }

        {
            if s.windows(2).all(|v| v != &['B', 'A']) {
                println!("B");
                continue;
            }
        }

        println!("A");
    }
}

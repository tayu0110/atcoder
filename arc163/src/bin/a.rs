use proconio::*;

fn main() {
    input! {t: usize}

    'base: for _ in 0..t {
        input! {n: usize, s: marker::Chars}

        let c = s[0];
        for i in 1..n {
            if c < s[i] {
                println!("Yes");
                continue 'base;
            }
        }

        if s.iter().skip(1).all(|&nc| nc < c) {
            println!("No");
            continue;
        }

        let mut t = vec![c];
        for i in 1..n {
            if s[i] == c {
                if &t[..] < &s[i..] {
                    println!("Yes");
                    continue 'base;
                }
            }

            t.push(s[i]);
        }

        println!("No");
    }
}

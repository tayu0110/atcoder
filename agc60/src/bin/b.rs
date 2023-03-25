use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {_n: usize, _m: usize, k: usize, s: Chars}

        let mut f = false;
        let mut cost = 0;
        for (i, c) in s.iter().enumerate().skip(1) {
            if c != &s[i - 1] {
                cost += 1;

                if f && i >= 2 && c == &s[i - 2] {
                    cost -= 1;
                    f = false;
                } else {
                    f = true;
                }
            } else {
                f = false;
            }
        }

        if k >= cost {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

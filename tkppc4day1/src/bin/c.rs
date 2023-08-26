use proconio::*;

fn main() {
    input! {n: u128, x: marker::Chars}

    for i in 2..=10 {
        let mut base = 1;
        let mut res = 0;
        for &c in x.iter().rev() {
            res += base * (c as u128 - b'0' as u128);
            base *= i;

            if res > n {
                break;
            }
        }

        if res == n {
            println!("{}", i);
            return;
        }
    }
}

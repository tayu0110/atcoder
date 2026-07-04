use math::MathInt;
use proconio::*;

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {k: usize, s: marker::Bytes}

        let mut m = 0;
        for &c in &s {
            m *= 10;
            m += c as usize - b'0' as usize;
            m %= k;
        }

        if m == 0 {
            println!("{}", s.into_iter().map(|b| b as char).collect::<String>());
            continue;
        }

        let mut ret = "".to_string();
        for t in 0..12 {
            if k < 10usize.pow(t) {
                break;
            }
            let r = (2 * k - 1 - m) % k * 10usize.inverse_mod(k).unwrap();
        }
    }
}

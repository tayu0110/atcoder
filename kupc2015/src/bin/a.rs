use proconio::*;

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {s: String}

        let mut s = &s[..];
        let mut res = 0;
        while s.len() >= 5 {
            if &s[..5] == "tokyo" || &s[..5] == "kyoto" {
                res += 1;
                s = &s[5..];
            } else {
                s = &s[1..];
            }
        }

        println!("{res}");
    }
}

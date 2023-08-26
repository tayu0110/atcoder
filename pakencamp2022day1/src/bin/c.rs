use proconio::*;

const MOD: usize = 998244353;

fn main() {
    input! {s: marker::Chars}

    let len = s.len();
    let mut now = 1;
    for i in 0..len {
        if i > len - 1 - i {
            break;
        }
        if s[i] != s[len - 1 - i] && s[i] != '?' && s[len - 1 - i] != '?' {
            now = 0;
            break;
        }

        if s[i] != '?' || s[len - 1 - i] != '?' {
            continue;
        }

        now *= 26;
        now %= MOD;
    }

    println!("{}", now);
}

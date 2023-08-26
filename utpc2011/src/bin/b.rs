use proconio::*;

fn main() {
    input! {s: marker::Chars}

    let mut res = 0;
    let len = s.len();
    for i in 0..len {
        let j = len - 1 - i;
        if j < i {
            break;
        }

        if s[i] != '(' && s[i] != ')' && s[i] == s[j] {
            continue;
        }

        if s[i] == '(' && s[j] == ')' {
            continue;
        }

        if s[i] == ')' && s[j] == '(' {
            continue;
        }

        res += 1;
    }

    println!("{}", res)
}

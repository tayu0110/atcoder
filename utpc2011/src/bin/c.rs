use proconio::*;

fn main() {
    input! {s: marker::Chars}

    let len = s.len();
    let mut res = 0;
    'base: for i in 0..1 << len {
        let s = s
            .iter()
            .enumerate()
            .filter(|(j, _)| i & (1 << j) != 0)
            .map(|(_, &c)| c)
            .collect::<Vec<_>>();
        let len = s.len();

        if !s
            .windows(3)
            .any(|v| vec!['i', 'w', 'i'].into_iter().zip(v).all(|(c, &d)| c == d))
        {
            continue;
        }

        for i in 0..len {
            let j = len - i - 1;
            if j < i {
                break;
            }

            if s[i] == '[' && s[j] == ']' {
                continue;
            }

            if s[i] == ']' && s[j] == '[' {
                continue;
            }

            if s[i] == '(' && s[j] == ')' {
                continue;
            }

            if s[i] == ')' && s[j] == '(' {
                continue;
            }

            if s[i] == '{' && s[j] == '}' {
                continue;
            }

            if s[i] == '}' && s[j] == '{' {
                continue;
            }

            if (s[i] == 'i' || s[i] == 'w') && (s[i] == s[j]) {
                continue;
            }
            continue 'base;
        }

        res = res.max(len);
    }

    println!("{}", res)
}

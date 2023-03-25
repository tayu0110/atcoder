use proconio::{marker::Chars, *};

fn main() {
    input! {h: usize, w: usize, mut s: [Chars; h]}

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }

            let mut t = 0;
            for k in i.saturating_sub(1)..(i + 2).min(h) {
                for l in j.saturating_sub(1)..(j + 2).min(w) {
                    if s[k][l] == '#' {
                        t += 1;
                    }
                }
            }

            s[i][j] = (t as u8 + b'0') as char;
        }
    }

    for s in s {
        println!("{}", s.into_iter().collect::<String>())
    }
}

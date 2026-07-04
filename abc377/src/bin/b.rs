use proconio::*;

const N: usize = 8;

fn main() {
    input! {mut s: [marker::Chars; N]}

    for i in 0..N {
        for j in 0..N {
            if s[i][j] == '#' {
                for k in 0..N {
                    if s[i][k] == '.' {
                        s[i][k] = 'x';
                    }
                    if s[k][j] == '.' {
                        s[k][j] = 'x';
                    }
                }
            }
        }
    }

    println!("{}", s.into_iter().flatten().filter(|&s| s == '.').count())
}

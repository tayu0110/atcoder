use proconio::input;
use proconio::marker::Chars;

fn main() {
    const LEN: usize = 9;
    input! {s: [Chars; LEN]}

    let mut res = 0;
    for i in 0..LEN {
        for j in 0..LEN {
            if s[i][j] == '.' {
                continue;
            }
            for k in i..LEN {
                for l in j + 1..LEN {
                    if s[k][l] == '.' {
                        continue;
                    }

                    let dx = l - j;
                    let dy = k - i;

                    if dy > j {
                        continue;
                    }

                    if k + dx >= LEN {
                        continue;
                    }

                    let (w, x) = (i + dx, j - dy);
                    let (u, v) = (k + dx, l - dy);

                    if s[w][x] == '#' && s[u][v] == '#' {
                        res += 1;
                    }
                }
            }
        }
    }

    println!("{}", res);
}

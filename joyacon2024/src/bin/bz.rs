use itertools::Itertools;
use proconio::*;

fn main() {
    input! {h: usize, w: usize, mut s: [marker::Chars; h]}

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }
            let mut cnt = 0;
            for dx in [0, 1, !0] {
                for dy in [0, 1, !0] {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let ni = i.wrapping_add(dx);
                    let nj = j.wrapping_add(dy);

                    if ni < h && nj < w {
                        cnt += (s[ni][nj] == '#') as u8;
                    }
                }
            }

            s[i][j] = (cnt + b'0') as char;
        }
    }

    for v in s {
        println!("{}", v.iter().join(""))
    }
}

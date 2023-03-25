use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {h: usize, w: usize, s: [Chars; h]}

    let mut nt = std::collections::BinaryHeap::new();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }
            if j == 0 || s[i][j-1] == '#' {
                let mut cnt = 0;
                let mut k = 0;
                while k < w && s[i][k] == '.' {
                    k += 1;
                    cnt += 1;
                }
                nt.push(std::cmp::Reverse((cnt, i, j, 0, 1)));
            }
            if i == 0 || s[i-1][j] == '#' {
                let mut cnt = 0;
                let mut k = 0;
                while k < h && s[k][j] == '.' {
                    k += 1;
                    cnt += 1;
                }
                nt.push(std::cmp::Reverse((cnt, i, j, 1, 0)));
            }
        }
    }

    
}
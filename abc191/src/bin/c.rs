#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {h: usize, w: usize, s: [Chars; h]}

    let mut res = 0;
    for i in 0..h-1 {
        for j in 0..w-1 {
            let mut cnt = 0;
            for k in 0..2 {
                for l in 0..2 {
                    if s[i+k][j+l] == '.' {
                        cnt += 1;
                    }
                }
            }

            if cnt == 1 || cnt == 3 {
                res += 1;
            }
        }
    }

    println!("{}", res);
}

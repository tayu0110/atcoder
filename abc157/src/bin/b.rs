#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {a: [[i32; 3]; 3], n: usize, b: [i32; n]}

    let mut card = vec![vec![false; 6]; 6];
    for b in b {
        for i in 0..3 {
            for j in 0..3 {
                if a[i][j] == b {
                    card[i][j] = true;
                }
            }
        }
    }

    for i in 0..3 {
        for j in 0..3 {
            for (di, dj) in [(0, 1), (1, 0), (1, 1)] {
                let (mut i, mut j) = (i, j);
                let mut cnt = 0;
                for _ in 0..3 {
                    if card[i][j] {
                        cnt += 1;
                    }
                    i += di;
                    j += dj;
                }

                if cnt == 3 {
                    println!("Yes");
                    return;
                }
            }

            if i == 0 && j == 2 && card[i][j] && card[i + 1][j - 1] && card[i + 2][j - 2] {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}

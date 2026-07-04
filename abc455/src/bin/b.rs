use proconio::*;

fn main() {
    input! {h: usize, w: usize, s: [marker::Bytes; h]}

    let mut ret = 0;
    for i in 0..h {
        for j in 0..w {
            for r in (1..=h).take_while(|r| r + i <= h) {
                for c in (1..=w).take_while(|c| c + j <= w) {
                    let (h1, h2, w1, w2) = (i, i + r, j, j + c);
                    let mut bad = false;
                    for i in h1..h2 {
                        for j in w1..w2 {
                            if s[i][j] != s[h1 + h2 - i - 1][w1 + w2 - j - 1] {
                                bad = true;
                            }
                        }
                    }
                    if !bad {
                        ret += 1;
                    }
                }
            }
        }
    }

    println!("{ret}")
}

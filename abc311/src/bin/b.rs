use proconio::*;

fn main() {
    input! {n: usize, d: usize, s: [marker::Chars; n]}

    let mut f = vec![false; d];
    for d in 0..d {
        let mut g = true;
        for i in 0..n {
            g &= s[i][d] == 'o';
        }
        f[d] = g;
    }

    let mut max = 0;
    for len in 1..=d {
        if f.windows(len).any(|v| v.iter().all(|f| *f)) {
            max = len;
        }
    }

    println!("{}", max)
}

use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {h: usize, w: usize, c: [Chars; h]}

    let mut res = vec![];
    for j in 0..w {
        let mut k = 0;
        for i in 0..h {
            if c[i][j] == '#' {
                k += 1;
            }
        }

        res.push(k)
    }

    println!("{}", res.iter().join(" "))
}

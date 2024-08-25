use std::borrow::Cow;

use proconio::*;

#[fastout]
fn main() {
    input! {n: usize, m: usize, q: usize, s: [marker::Bytes; n], q: [char; q]}

    let mut map = vec![Cow::Borrowed("NA"); 256];
    for i in 0..n {
        for j in 0..m {
            if s[i][j] != b'*' {
                map[s[i][j] as usize] = Cow::Owned(format!("{} {}", i + 1, j + 1));
            }
        }
    }

    for q in q {
        println!("{}", map[q as usize]);
    }
}

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {s: marker::Bytes, q: usize, k: [usize; q]}

    let mut res = vec![];
    for k in k {
        let t = (k - 1) / s.len();
        let k = (k - 1) % s.len();

        let rev = t.count_ones() % 2 != 0;

        if rev {
            if s[k].is_ascii_lowercase() {
                res.push(s[k].to_ascii_uppercase() as char);
            } else {
                res.push(s[k].to_ascii_lowercase() as char);
            }
        } else {
            res.push(s[k] as char)
        }
    }
    println!("{}", res.iter().join(" "))
}

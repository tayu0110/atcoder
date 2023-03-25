use proconio::*;

const S: &'static str = "1234567890abcdefghijklmnopqrstuvwxyz";

fn main() {
    input! {s: marker::Chars, k: marker::Chars}

    let not_contain = S.chars().filter(|c| !k.contains(c)).collect::<Vec<_>>();
    let m = not_contain.len();
    let mut l = s
        .iter()
        .filter(|&c| not_contain.contains(c))
        .collect::<Vec<_>>();
    l.sort();
    l.dedup();
    let l = l.len();

    let res = (0..m)
        .map(|i| {
            if i < l {
                i as f64 / (i as f64 + 1.0) * 2.0
            } else {
                l as f64 / (l as f64 + 1.0) * 2.0
            }
        })
        .sum::<f64>()
        + s.len() as f64;

    println!("{}", res);
}

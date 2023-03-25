use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {s: Chars}

    let map = "atcoder"
        .chars()
        .enumerate()
        .map(|(i, c)| (c, i))
        .collect::<std::collections::HashMap<_, _>>();
    let mut k = vec![];
    for c in s {
        k.push(*map.get(&c).unwrap());
    }

    let mut res = 0;
    for i in 0..k.len() {
        for j in 0..i {
            if k[j] > k[i] {
                res += 1;
            }
        }
    }

    println!("{}", res);
}

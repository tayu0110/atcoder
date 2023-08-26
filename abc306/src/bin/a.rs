use proconio::*;

fn main() {
    input! {_: usize, s: marker::Chars}
    let mut res = vec![];
    for c in s {
        res.push(c);
        res.push(c);
    }
    println!("{}", res.iter().collect::<String>())
}

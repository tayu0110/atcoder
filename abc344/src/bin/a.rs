use proconio::*;

fn main() {
    input! {s: String}

    let s = s.split('|').collect::<Vec<_>>();
    println!("{}{}", s[0], s[2]);
}

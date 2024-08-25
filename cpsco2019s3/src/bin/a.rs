use proconio::*;

fn main() {
    input! {s: String}

    let s = s.replace('A', "?");
    let s = s.replace('O', "A");
    println!("{}", s.replace('?', "O"))
}

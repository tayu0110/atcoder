use proconio::*;

fn main() {
    input! {_: usize,  s: String,  t: String}

    let s = s.replace("1", "l");
    let s = s.replace("0", "o");

    let t = t.replace("1", "l");
    let t = t.replace("0", "o");

    if s == t {
        println!("Yes")
    } else {
        println!("No")
    }
}

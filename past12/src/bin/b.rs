use proconio::*;

fn main() {
    input! {n: String}
    let len = n.len();

    if len < 3 {
        println!("0")
    } else {
        println!("{}", &n[..len - 2])
    }
}

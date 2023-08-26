use proconio::*;

fn main() {
    input! {a: i64, b: i64, c: u32}

    let mut c = c % 2;
    if c == 0 {
        c += 2;
    }

    let pa = a.pow(c);
    let pb = b.pow(c);
    if pa < pb {
        println!("<")
    } else if pa == pb {
        println!("=")
    } else {
        println!(">")
    }
}

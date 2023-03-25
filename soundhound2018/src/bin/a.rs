use proconio::*;

fn main() {
    input! {x: marker::Chars, y: marker::Chars}

    let mut res = String::new();
    res.push(x[0]);
    res.push(y[0]);

    if res == "SH" {
        println!("YES")
    } else {
        println!("NO")
    }
}

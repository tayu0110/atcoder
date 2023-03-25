use proconio::*;

fn main() {
    input! {o: marker::Chars, e: marker::Chars}

    let mut res = String::new();
    for i in 0..o.len() {
        res.push(o[i]);
        if i < e.len() {
            res.push(e[i]);
        }
    }

    println!("{}", res)
}

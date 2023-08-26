use proconio::*;

fn main() {
    input! {n: marker::Chars}

    if n.contains(&'7') {
        println!("Yes")
    } else {
        println!("No")
    }
}

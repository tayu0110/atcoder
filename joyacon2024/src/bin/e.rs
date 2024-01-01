use proconio::*;

fn main() {
    input! {n: marker::Chars}

    let mut m = n.clone();
    m.reverse();

    if n == m {
        println!("YES")
    } else {
        println!("NO")
    }
}

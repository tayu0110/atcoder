use proconio::*;

fn main() {
    input! {a: marker::Chars, b: marker::Chars}

    if a.len() > b.len() {
        println!("GREATER")
    } else if a.len() < b.len() {
        println!("LESS")
    } else {
        if a > b {
            println!("GREATER")
        } else if a < b {
            println!("LESS")
        } else {
            println!("EQUAL")
        }
    }
}

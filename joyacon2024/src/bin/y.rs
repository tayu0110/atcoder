use proconio::*;

fn main() {
    input! {c: char, d: char}

    if c.to_lowercase().next().unwrap() == d {
        println!("Yes")
    } else {
        println!("No")
    }
}

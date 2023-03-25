use proconio::*;

fn main() {
    input! {s: marker::Chars}

    println!(
        "{}",
        s.into_iter().position(|c| c.is_uppercase()).unwrap() + 1
    );
}

use proconio::*;

fn main() {
    input! {s: marker::Chars}

    println!(
        "{}",
        if s.chunks(2)
            .all(|c| c[1] == '0') { "Yes" } else { "No" }
    )
}

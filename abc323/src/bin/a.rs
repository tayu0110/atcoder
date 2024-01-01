use proconio::*;

fn main() {
    input! {s: marker::Chars}

    println!(
        "{}",
        s.chunks(2)
            .all(|c| c[1] == '0')
            .then_some("Yes")
            .unwrap_or("No")
    )
}

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {s: marker::Bytes}

    println!(
        "{}",
        s.into_iter()
            .map(|b| {
                (match b {
                    b'A' | b'T' => b ^ b'A' ^ b'T',
                    b'G' | b'C' => b ^ b'G' ^ b'C',
                    _ => {
                        unreachable!()
                    }
                }) as char
            })
            .join("")
    )
}

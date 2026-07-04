use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, s: [marker::Bytes; n]}

    println!(
        "{}",
        s.into_iter()
            .map(|s| match s[0] {
                b'a' | b'b' | b'c' => 2,
                b'd' | b'e' | b'f' => 3,
                b'g' | b'h' | b'i' => 4,
                b'j' | b'k' | b'l' => 5,
                b'm' | b'n' | b'o' => 6,
                b'p' | b'q' | b'r' | b's' => 7,
                b't' | b'u' | b'v' => 8,
                b'w' | b'x' | b'y' | b'z' => 9,
                _ => unreachable!(),
            })
            .join("")
    )
}

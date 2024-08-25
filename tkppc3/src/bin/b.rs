use proconio::*;

fn main() {
    input! {n: marker::Bytes}

    for rem in [6, 11] {
        println!(
            "{}",
            if n.iter().fold(0, |s, &v| (s * 10 + v - b'0') % rem) == 0 {
                "yES"
            } else {
                "nO"
            }
        );
    }
}

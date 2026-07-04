use proconio::*;

fn main() {
    input! {t: marker::Bytes, u: marker::Bytes}

    if t.windows(u.len()).any(|t| {
        t.iter()
            .zip(u.iter())
            .filter(|t| *t.0 != b'?')
            .all(|(t, u)| t == u)
    }) {
        println!("Yes")
    } else {
        println!("No")
    }
}

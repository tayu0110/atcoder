use proconio::*;

fn main() {
    input! {_: i32, s: marker::Bytes}

    println!(
        "{}",
        s.into_iter()
            .scan(0, |s, v| {
                if v == b'I' {
                    *s += 1;
                } else {
                    *s -= 1;
                }
                Some(*s)
            })
            .max()
            .unwrap()
            .max(0)
    )
}

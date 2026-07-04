use proconio::*;

fn main() {
    input! {_: usize, s: marker::Chars}

    println!("{}", s.windows(3).filter(|v| v == &['#', '.', '#']).count())
}

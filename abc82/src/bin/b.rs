use proconio::{marker::Chars, *};

fn main() {
    input! {mut s: Chars, mut t: Chars}

    s.sort();
    t.sort();
    t.reverse();

    let (s, t) = (
        s.into_iter().collect::<String>(),
        t.into_iter().collect::<String>(),
    );

    if s < t {
        println!("Yes")
    } else {
        println!("No")
    }
}

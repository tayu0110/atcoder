use proconio::*;

fn main() {
    input! {_: usize, s: marker::Chars}

    for v in s.windows(2) {
        if v[0] == v[1] {
            println!("No");
            return;
        }
    }

    println!("Yes")
}

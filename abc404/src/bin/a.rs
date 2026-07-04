use proconio::*;

fn main() {
    input! {s: String}

    for c in 'a'..='z' {
        if !s.contains(c) {
            println!("{c}");
            break;
        }
    }
}

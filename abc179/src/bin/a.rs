use proconio::*;

fn main() {
    input! {s: String}

    if s.ends_with("s") {
        println!("{}es", s);
    } else {
        println!("{}s", s)
    }
}

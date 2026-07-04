use proconio::*;

fn main() {
    input! {s: String, t: String}

    if s == t {
        println!("0");
        return;
    }

    println!(
        "{}",
        s.chars().zip(t.chars()).take_while(|(s, t)| s == t).count() + 1
    )
}

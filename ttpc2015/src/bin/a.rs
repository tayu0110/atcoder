use proconio::*;

fn main() {
    input! {s: String}
    let t = if s.contains(&"B") {
        "Bachelor"
    } else if s.contains(&"M") {
        "Master"
    } else {
        "Doctor"
    };
    println!("{} {}", t, &s[..2]);
}

use proconio::*;

fn main() {
    input! {s: String}
    let t = ["SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT"];

    println!("{}", 7 - t.iter().position(|&u| u == &s).unwrap())
}

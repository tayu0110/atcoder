use proconio::*;

fn main() {
    input! {s: [String]}

    if s.iter().all(|s| s == "Perfect") {
        println!("All Perfect")
    } else if s.iter().all(|s| s == "Perfect" || s == "Great") {
        println!("Full Combo")
    } else {
        println!("Failed")
    }
}

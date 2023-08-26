use proconio::*;

fn main() {
    input! {s: String}

    if s == "ACE"
        || s == "BDF"
        || s == "CEG"
        || s == "DFA"
        || s == "EGB"
        || s == "FAC"
        || s == "GBD"
    {
        println!("Yes")
    } else {
        println!("No")
    }
}

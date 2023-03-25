use proconio::input;

fn main() {
    input! {s: String}

    if s == "RRR" {
        println!("3");
    } else if s == "RRS" || s == "SRR" {
        println!("2");
    } else if s == "SSS" {
        println!("0");
    } else {
        println!("1");
    }
}

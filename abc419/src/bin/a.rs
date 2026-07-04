use proconio::*;

fn main() {
    input! {s: String}

    let res = match s.as_str() {
        "red" => "SSS",
        "blue" => "FFF",
        "green" => "MMM",
        _ => "Unknown",
    };
    println!("{res}")
}

#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {s: String};
    println!("{}",
        match s.as_str() {
            "Sunny" => "Cloudy",
            "Cloudy" => "Rainy",
            _ => "Sunny"
        })
}

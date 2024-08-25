use proconio::*;
use regex::Regex;

fn main() {
    input! {s: String}

    if Regex::new("[^M]*R.*M.*").unwrap().is_match(&s) {
        println!("Yes")
    } else {
        println!("No")
    }
}

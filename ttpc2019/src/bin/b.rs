use proconio::*;
use regex::Regex;

fn main() {
    input! {n: usize, s: [String; n]}

    let re = Regex::new(".*okyo.*ech.*").unwrap();
    s.into_iter().for_each(|s| {
        if re.is_match(&s) {
            println!("Yes")
        } else {
            println!("No")
        }
    })
}

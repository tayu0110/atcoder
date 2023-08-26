use proconio::*;

fn main() {
    input! {c: char}

    if "aeiou".chars().any(|d| d == c) {
        println!("vowel")
    } else {
        println!("consonant")
    }
}

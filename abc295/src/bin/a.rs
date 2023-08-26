use proconio::*;

fn main() {
    input! {n: usize, s: [String; n]}

    for s in s {
        if s == "and" || s == "not" || s == "that" || s == "the" || s == "you" {
            println!("Yes");
            return;
        }
    }

    println!("No")
}

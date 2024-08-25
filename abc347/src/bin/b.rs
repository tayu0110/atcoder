use proconio::*;
use rustc_hash::FxHashSet;

fn main() {
    input! {s: marker::Chars}

    let mut set = FxHashSet::default();
    for i in 0..s.len() {
        for j in i + 1..=s.len() {
            set.insert(s[i..j].into_iter().collect::<String>());
        }
    }

    println!("{}", set.len())
}

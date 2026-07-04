use itertools::Itertools;
use proconio::*;

fn main() {
    input! {a: [i32; 5]}

    let mut t = [0; 128];
    for (b, a) in b"ABCDE".into_iter().zip(a) {
        t[*b as usize] = a;
    }

    let mut member = vec![];
    for i in 1..1 << 5 {
        let mut s = String::new();
        for j in 0..5 {
            if i & (1 << j) != 0 {
                s.push((b'A' + j) as char);
            }
        }
        member.push(s);
    }

    member.sort_unstable_by_key(|s| (-s.chars().fold(0, |s, v| s + t[v as usize]), s.clone()));
    println!("{}", member.iter().join("\n"))
}

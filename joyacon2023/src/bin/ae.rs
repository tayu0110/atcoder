use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {n: usize, s: [Chars; n]}

    let mut set = std::collections::HashSet::new();
    let p = "HDCS";
    let q = "A23456789TJQK";
    for s in s {
        if !p.contains(s[0]) {
            println!("No");
            return;
        }

        if !q.contains(s[1]) {
            println!("No");
            return;
        }

        if set.contains(&s) {
            println!("No");
            return;
        }

        set.insert(s);
    }

    println!("Yes");
}

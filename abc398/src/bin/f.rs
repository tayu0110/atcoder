use proconio::*;
use string::DynamicRollingHash;

fn main() {
    input! {s: String}

    let n = s.len();
    let mut hash = DynamicRollingHash::new(&s);
    for i in 0..n {
        if hash.is_palindrome(i..) {
            let mut t = s.clone();
            let rev = s.chars().take(i).collect::<String>();
            t.extend(rev.chars().rev());
            println!("{t}");
            return;
        }
    }

    let mut t = s.clone();
    t.extend(s.chars().rev());
    println!("{t}")
}

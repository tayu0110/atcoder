use proconio::*;

fn main() {
    input! {s: marker::Bytes}
    let len = s.len();

    let mut memo = vec![false; len + 1];
    memo[0] = true;
    for i in 0..len {
        if !memo[i] {
            continue;
        }

        for ns in vec!["dream", "dreamer", "erase", "eraser"] {
            if i + ns.len() > len {
                continue;
            }

            if &ns.as_bytes()[..] == &s[i..i + ns.len()] {
                memo[i + ns.len()] = true;
            }
        }
    }

    if memo[len] {
        println!("YES")
    } else {
        println!("NO")
    }
}

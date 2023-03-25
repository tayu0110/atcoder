use proconio::{input, marker::Chars};

fn main() {
    input! {s: Chars}

    let mut prev_open = vec![];
    let mut cs = vec![];
    for (i, c) in s.into_iter().enumerate() {
        if c == '(' {
            prev_open.push(i);
        } else if c == ')' {
            let prev = prev_open.pop().unwrap();
            while let Some((j, d)) = cs.pop() {
                if j <= prev {
                    cs.push((j, d));
                    break;
                }
            }
        } else {
            if cs.iter().any(|(_, d)| *d == c) {
                println!("No");
                return;
            }
            cs.push((i, c));
        }
    }

    println!("Yes");
}

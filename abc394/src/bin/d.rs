use proconio::*;

fn main() {
    input! {s: marker::Bytes}

    let mut stack = vec![];
    for s in s {
        match stack.last().copied() {
            None => stack.push(s),
            Some(c) if c == b'(' && s == b')' => {
                stack.pop();
            }
            Some(c) if c == b'[' && s == b']' => {
                stack.pop();
            }
            Some(c) if c == b'<' && s == b'>' => {
                stack.pop();
            }
            Some(_) => stack.push(s),
        }
    }

    if stack.is_empty() {
        println!("Yes")
    } else {
        println!("No")
    }
}

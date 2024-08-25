use proconio::*;

fn main() {
    input! {s: marker::Chars}

    if s.len() == 1 {
        println!("{}", s[0]);
        return;
    }

    let mut stack = vec![];
    for &c in &s {
        match stack.last() {
            Some(&pc) if pc == c => {
                stack.pop();
            }
            _ => stack.push(c),
        }
    }
    if stack.is_empty() {
        if s[0] == *s.last().unwrap() {
            println!();
        } else {
            println!("{}{}", s[0], s[0]);
        }
        return;
    }

    if s[0] != stack[0] {
        stack.insert(0, s[0]);
        stack.insert(0, s[0]);
    }

    stack.reverse();
    println!("{}", stack.into_iter().collect::<String>())
}

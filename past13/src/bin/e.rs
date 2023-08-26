use proconio::*;

fn main() {
    input! {s: marker::Chars}

    let mut stack = vec![];
    for c in s {
        match stack.last() {
            Some(&last) if last == '(' && c == ')' => {
                stack.pop();
            }
            _ => stack.push(c),
        }
    }

    if stack.is_empty() {
        println!("Yes")
    } else {
        println!("No")
    }
}

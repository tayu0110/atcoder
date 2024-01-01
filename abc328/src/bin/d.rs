use proconio::*;

fn main() {
    input! {s: marker::Bytes}

    let mut stack = vec![];
    for c in s {
        if stack.len() < 2 {
            stack.push(c);
            continue;
        }

        let len = stack.len();
        if c == b'C' && &stack[len - 2..len] == &[b'A', b'B'] {
            stack.pop();
            stack.pop();
        } else {
            stack.push(c);
        }
    }

    println!(
        "{}",
        stack.into_iter().map(|c| c as char).collect::<String>()
    )
}

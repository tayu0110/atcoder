use proconio::*;

fn main() {
    input! {_: usize, s: marker::Bytes}

    let mut start = usize::MAX;
    let mut stack = vec![];
    let mut res = String::new();
    for c in s {
        if c == b'/' && stack.last() == Some(&b'*') && start.saturating_add(2) <= stack.len() {
            stack.truncate(start - 1);
            start = usize::MAX;
            res.extend(stack.drain(..).map(|c| c as char));
        } else if c == b'*' && stack.last() == Some(&b'/') {
            start = start.min(stack.len());
            stack.push(c);
        } else {
            stack.push(c);
        }
    }
    res.extend(stack.drain(..).map(|c| c as char));

    println!("{}", res)
}

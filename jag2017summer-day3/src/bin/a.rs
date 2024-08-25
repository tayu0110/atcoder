use proconio::*;

fn main() {
    input! {s: marker::Bytes}

    let mut stack = vec![];
    for s in s {
        if s == b')' {
            stack.pop();
        } else if s == b'(' {
            stack.push(s);
        } else {
            break;
        }
    }
    println!("{}", stack.len());
}

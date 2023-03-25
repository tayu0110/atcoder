#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {s: Chars};

    let n = s.len();

    let mut stack = std::collections::VecDeque::<char>::new();

    for c in s {
        if stack.is_empty() {
            stack.push_back(c);
        } else {
            if stack.back().unwrap() != &c {
                stack.pop_back().unwrap();
            } else {
                stack.push_back(c);
            }
        }
    }

    println!("{}", n - stack.len());
}

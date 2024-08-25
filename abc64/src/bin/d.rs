use proconio::{input, marker::Chars};

fn main() {
    input! {_: usize, s: Chars}

    let mut stack = vec![];
    for &c in &s {
        if c == '(' {
            stack.push(c);
        } else if stack.is_empty() {
            stack.push(c);
        } else if stack.last().unwrap() == &')' {
            stack.push(c);
        } else {
            stack.pop().unwrap();
        }
    }

    let close = stack.iter().filter(|&&c| c == ')').count();
    let open = stack.iter().filter(|&&c| c == '(').count();

    println!(
        "{}",
        (0..close)
            .map(|_| '(')
            .chain(s.into_iter())
            .chain((0..open).map(|_| ')'))
            .collect::<String>()
    )
}

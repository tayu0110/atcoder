use proconio::*;

fn main() {
    input! {_n: usize, s: marker::Chars}

    let mut stack = vec![];
    let mut open = 0;
    for c in s {
        if c == '(' {
            stack.push(c);
            open += 1;
        } else if c == ')' {
            if open > 0 {
                while let Some(c) = stack.pop() {
                    if c == '(' {
                        break;
                    }
                }
                open -= 1;
            } else {
                stack.push(c);
            }
        } else {
            stack.push(c);
        }
    }

    println!("{}", stack.iter().collect::<String>())
}

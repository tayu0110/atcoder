use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut stack = vec![];
    for a in a {
        if stack.len() >= 3 && &stack[stack.len() - 3..] == &[a, a, a] {
            stack.truncate(stack.len() - 3);
        } else {
            stack.push(a);
        }
    }

    println!("{}", stack.len());
}

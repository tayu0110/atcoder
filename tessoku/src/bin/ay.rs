use proconio::*;

fn main() {
    input! {q: usize}

    let mut stack = vec![];
    for _ in 0..q {
        input! {ty: usize}

        if ty == 1 {
            input! {x: String}
            stack.push(x);
        } else if ty == 2 {
            println!("{}", stack.last().unwrap());
        } else {
            stack.pop();
        }
    }
}

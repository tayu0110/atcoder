use proconio::*;

fn main() {
    input! {q: usize}

    let mut min = 0;
    let mut stack = vec![0i32];
    for _ in 0..q {
        input! {ty: u8}

        if ty == 1 {
            input! {c: char}
            let last = stack.last().unwrap() + if c == '(' { 1 } else { -1 };
            stack.push(last);
            if last < 0 {
                min += 1;
            }
        } else {
            let last = stack.pop().unwrap();
            if last < 0 {
                min -= 1;
            }
        }
        if min == 0 && stack.last().unwrap() == &0 {
            println!("Yes")
        } else {
            println!("No")
        }
    }
}

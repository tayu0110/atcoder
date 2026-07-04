use proconio::*;

fn solve(a: &[u8]) -> Vec<u8> {
    let mut stack = vec![];
    for &a in a {
        stack.push(a);

        if stack.ends_with(b"(xx)") {
            let len = stack.len();
            stack.truncate(len - 4);
            stack.extend(b"xx");
        }
    }

    stack
}

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {a: marker::Bytes, b: marker::Bytes}

        if a == b {
            println!("Yes");
            continue;
        }

        let na = solve(&a);
        let nb = solve(&b);
        if na == nb {
            println!("Yes");
        } else {
            println!("No")
        }
    }
}

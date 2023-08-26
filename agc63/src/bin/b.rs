use proconio::*;

fn main() {
    input! {n: usize, mut a: [usize; n]}
    a.push(std::usize::MAX);

    let mut res = 0usize;
    let mut stack = vec![n];
    for (i, &c) in a.iter().enumerate().rev() {
        if c == 1 {
            let mut now = 1;
            while let Some(top) = stack.pop() {
                if now + 1 != a[top] {
                    stack.push(top);
                    res += top - i;
                    break;
                }
                now += 1;
            }
        } else {
            stack.push(i);
        }
    }

    println!("{}", res);
}

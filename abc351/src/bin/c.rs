use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut stack = vec![];
    for a in a {
        stack.push(a);
        while stack.len() > 1 {
            let k = stack.pop().unwrap();
            let l = stack.pop().unwrap();

            if k != l {
                stack.push(l);
                stack.push(k);
                break;
            }

            stack.push(k + 1);
        }
    }

    println!("{}", stack.len())
}

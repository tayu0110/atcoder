use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, h: [usize; n]}

    let mut stack = vec![];
    let mut cum = 0;
    let mut res = vec![];
    for h in h {
        cum += h;
        let mut c = 1;
        while let Some((ph, cnt)) = stack.pop() {
            if ph < h {
                c += cnt;
                cum += (h - ph) * cnt;
            } else if ph == h {
                stack.push((h, c + cnt));
                break;
            } else {
                stack.push((ph, cnt));
                stack.push((h, c));
                break;
            }
        }

        if stack.is_empty() {
            stack.push((h, c));
        }

        res.push(cum + 1);
    }

    println!("{}", res.iter().join(" "))
}

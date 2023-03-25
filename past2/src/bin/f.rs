use proconio::input;

fn main() {
    input! {n: usize, mut p: [(usize, usize); n]}
    p.sort_by_key(|(a, _)| std::cmp::Reverse(*a));

    let mut sum = 0;
    let mut res = vec![];
    let mut rem = std::collections::BinaryHeap::new();
    for i in 1..=n {
        let mut buf = vec![];
        while let Some((a, b)) = p.pop() {
            if a > i {
                p.push((a, b));
                break;
            } else if a == i {
                buf.push(b);
            }
        }
        buf.sort();
        if let Some(max) = buf.pop() {
            if let Some(k) = rem.pop() {
                if k > max {
                    sum += k;
                    rem.push(max);
                } else {
                    sum += max;
                    rem.push(k);
                }
            } else {
                sum += max;
            }
        } else {
            if let Some(k) = rem.pop() {
                sum += k;
            }
        }

        res.push(sum);
        while let Some(k) = buf.pop() {
            rem.push(k);
        }
    }

    for res in res {
        println!("{}", res);
    }
}

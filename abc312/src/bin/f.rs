use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::*;

fn main() {
    input! {n: usize, mut m: usize, p: [(usize, usize); n]}

    let mut not_required = BinaryHeap::new();
    let mut required = vec![];
    let mut knife = vec![];
    for (t, x) in p {
        if t == 0 {
            not_required.push(Reverse(x));
        } else if t == 1 {
            required.push(x);
        } else {
            knife.push(x);
        }
    }

    required.sort();
    knife.sort();

    while not_required.len() > m {
        not_required.pop();
    }

    let mut res = not_required.iter().map(|Reverse(v)| v).sum::<usize>();
    let mut now = res;
    while let Some(mut rem) = knife.pop() {
        if m == 0 {
            break;
        }
        m -= 1;
        while not_required.len() > m {
            let Reverse(v) = not_required.pop().unwrap();
            now -= v;
        }

        while rem > 0 {
            if required.is_empty() {
                break;
            }
            let next = required.pop().unwrap();
            if not_required.len() == m {
                let Reverse(v) = not_required.pop().unwrap();
                now -= v;
            }
            now += next;
            res = res.max(now);
            not_required.push(Reverse(next));
            rem -= 1;
        }
    }

    println!("{}", res);

    // let mut rem = 0;
    // while not_required.len() < m {
    //     if rem == 0 {
    //         if knife.is_empty() || not_required.len() + 1 >= m {
    //             break;
    //         }

    //         rem += knife.pop().unwrap();
    //         m -= 1;
    //     }

    //     if required.is_empty() {
    //         break;
    //     }
    //     not_required.push((required.pop().unwrap(), 1));
    //     rem -= 1;
    // }
    // not_required.sort();
    // not_required.reverse();

    // let mut res = not_required.iter().map(|v| v.0).sum::<usize>();
    // let mut now = res;
    // let mut buf = not_required
    //     .into_iter()
    //     .map(|v| Reverse(v))
    //     .collect::<BinaryHeap<_>>();

    // loop {
    //     if rem == 0 {
    //         if buf.is_empty() || knife.is_empty() {
    //             break;
    //         }

    //         let Reverse(pop) = buf.pop().unwrap();
    //         if pop.1 == 1 {
    //             break;
    //         }
    //         now -= pop.0;
    //         rem += knife.pop().unwrap();
    //     }

    //     if buf.is_empty() {
    //         break;
    //     }

    //     let Reverse((val, ty)) = buf.pop().unwrap();
    //     if ty == 1 {
    //         break;
    //     }
    //     if required.is_empty() {
    //         break;
    //     }

    //     let re = required.pop().unwrap();

    //     if re < val {
    //         break;
    //     }

    //     buf.push(Reverse((re, 1)));
    //     rem -= 1;

    //     now += re;
    //     now -= val;

    //     res = res.max(now);
    //     eprintln!("res: {res}, now: {now}, rem: {rem}, buf: {buf:?}, req: {required:?}");
    // }

    // println!("{}", res);
}

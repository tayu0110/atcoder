use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize,}

    let mut t = (0..=n).collect::<Vec<_>>();
    let mut now = n;
    let mut top = vec![n];
    let mut back = vec![n - 1];
    t[n] -= 1;
    t[n - 1] -= 1;
    for i in 0..n * (n + 1) / 2 / 2 - 1 {
        let (f, b) = if i % 2 == 0 {
            (&mut top, &mut back)
        } else {
            (&mut back, &mut top)
        };

        while t[now] == 0 {
            now -= 1;
        }

        for v in vec![f, b] {
            let mut k = now;
            while k > 0 {
                if t[k] > 0 && *v.last().unwrap() != k {
                    break;
                }
                k -= 1;
            }

            v.push(k);
            t[k] -= 1;
        }

        // eprintln!("now: {now}, top: {top:?}, back: {back:?}, t: {t:?}");
    }

    if t[1] > 0 {
        top.push(1);
    } else if t[2] > 0 {
        top.push(2);
    } else if t[3] > 0 {
        top.push(3);
    }
    back.reverse();
    top.extend(back);
    assert_eq!(top.len(), n * (n + 1) / 2);
    println!("{}", top.iter().join(" "))
}

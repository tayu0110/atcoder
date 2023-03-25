use std::collections::HashSet;

use proconio::{marker::Chars, *};

const MAX: i32 = 8000;

fn solve(x: i32, dx: &[i32]) -> bool {
    let mut memo = HashSet::new();
    memo.insert(0);
    for &dx in dx {
        let mut new = HashSet::new();
        for &k in &memo {
            if k + dx <= MAX * 2 {
                new.insert(k + dx);
            }
            if k - dx >= -MAX * 2 {
                new.insert(k - dx);
            }
        }

        memo = new;
    }

    memo.contains(&x)
}

fn main() {
    input! {s: Chars, mut x: i32, y: i32}

    for _ in s.iter().take_while(|&&c| c == 'F') {
        x -= 1;
    }

    let s = s.into_iter().skip_while(|&c| c == 'F').collect::<Vec<_>>();
    let mut f = false;
    let (mut mx, mut my) = (vec![], vec![]);
    let mut mv = 0;
    for c in s {
        if c == 'F' {
            mv += 1;
        } else {
            if mv != 0 {
                if f {
                    my.push(mv);
                } else {
                    mx.push(mv);
                }
            }
            mv = 0;
            f = !f;
        }
    }

    if mv != 0 {
        if f {
            my.push(mv);
        } else {
            mx.push(mv);
        }
    }

    if solve(x.abs(), &mx) && solve(y.abs(), &my) {
        println!("Yes");
    } else {
        println!("No");
    }
}

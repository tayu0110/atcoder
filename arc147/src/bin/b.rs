use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, p: [usize; n]};
    let mut p = p.into_iter().map(|v| v-1).collect_vec();

    let mut pos = p.iter().enumerate().map(|(i, v)| (*v, i)).collect_vec();
    pos.sort();
    let mut pos = pos.into_iter().map(|(_, i)| i).collect_vec();

    let mut res = vec![];
    for i in 0..n {
        let mut now = pos[i];
        while now != i {
            if now % 2 != i % 2 {
                let mut ok = false;
                for j in i..now {
                    if p[j] % 2 != j % 2 {
                        ok = true;
                        break;
                    }
                }
                if !ok {
                    loop {
                        if p[now+1] % 2 != (now+1) % 2 {
                            res.push(format!("A {}", now+1));
                            pos.swap(p[now], p[now+1]);
                            p.swap(now, now+1);
                            now += 1;
                            break;
                        }
                        res.push(format!("B {}", now+1));
                        pos.swap(p[now], p[now+2]);
                        p.swap(now, now+2);
                        now += 2;
                    }
                }
            }
            if now % 2 == i % 2 {
                res.push(format!("B {}", now-1));
                pos.swap(p[now], p[now-2]);
                p.swap(now, now-2);
                now -= 2;
            } else if p[now-1] % 2 != (now-1) % 2 {
                res.push(format!("A {}", now));
                pos.swap(p[now], p[now-1]);
                p.swap(now, now-1);
                now -= 1;
            } else {
                res.push(format!("B {}", now-1));
                pos.swap(p[now], p[now-2]);
                p.swap(now, now-2);
                now -= 2;    
            }
        }
    }

    println!("{}", res.len());
    for v in res {
        println!("{}", v);
    }
}

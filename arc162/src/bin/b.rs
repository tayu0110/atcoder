use proconio::*;

fn main() {
    input! {n: usize, mut p: [usize; n]}

    if n == 2 {
        if p[0] == 1 {
            println!("Yes");
            println!("0");
        } else {
            println!("No");
        }
        return;
    }

    {
        let mut r = 0;
        for i in 0..n {
            for j in 0..i {
                if p[j] > p[i] {
                    r += 1;
                }
            }
        }

        if r % 2 == 1 {
            println!("No");
            return;
        }
    }

    const MAX: usize = 2000;
    let mut done = 0;
    let mut res = vec![];
    for _ in 0..MAX {
        let now = done + 1;

        for i in 0..n {
            if p[i] == now {
                if i + 1 == now {
                    break;
                }

                let tar = now - 1;
                if i == n - 1 {
                    res.push((n - 1, n - 3));
                    res.push((n - 1, tar));
                    let b = p.remove(n - 3);
                    let f = p.pop().unwrap();
                    p.insert(tar, b);
                    p.insert(tar, f);
                } else {
                    let (b, f) = (p.remove(i + 1), p.remove(i));
                    p.insert(tar, b);
                    p.insert(tar, f);

                    res.push((i + 1, tar));
                }

                break;
            }
        }
        done += 1;

        if done == n {
            println!("Yes");
            println!("{}", res.len());
            for (l, r) in res {
                println!("{} {}", l, r);
            }
            return;
        }
    }

    unreachable!()
}

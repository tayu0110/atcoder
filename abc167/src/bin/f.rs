use proconio::*;

fn main() {
    input! {n: usize, s: [marker::Chars; n]}

    let mut d = vec![];
    for s in s {
        let mut buf = vec![];
        for c in s {
            match buf.last() {
                Some(&p) if p == '(' && c == ')' => {
                    buf.pop().unwrap();
                }
                _ => buf.push(c),
            }
        }

        let mut min = 0i32;
        let mut now = 0i32;
        for &c in &buf {
            if c == '(' {
                now += 1;
            } else {
                now -= 1;
            }
            min = min.min(now);
        }
        d.push((now, min));
    }

    d.sort_by(|&l, &r| {
        if l.0 >= 0 && l.1 >= 0 {
            if r.0 >= 0 {
                l.1.cmp(&r.1)
            } else {
                l.0.cmp(&r.0)
            }
        } else if l.0 >= 0 && l.1 < 0 {
            if r.0 >= 0 {
                l.1.cmp(&r.1)
            } else {
                l.0.cmp(&r.0)
            }
        } else {
            l.cmp(&r)
        }
    });

    let mut now = 0;
    while let Some((last, min)) = d.pop() {
        if now + min < 0 {
            println!("No");
            return;
        }

        now += last;
    }
    if now == 0 {
        println!("Yes")
    } else {
        println!("No")
    }
}

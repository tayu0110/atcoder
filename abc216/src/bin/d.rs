use proconio::*;

fn main() {
    input! {n: usize, m: usize}

    let mut nt = vec![];
    let mut balls = vec![vec![]; n + 1];
    let mut t = vec![];
    for i in 0..m {
        input! {a: [usize]}

        let last = *a.last().unwrap();
        balls[last].push(i);
        if balls[last].len() == 2 {
            nt.push(last);
        }
        t.push(a);
    }

    while let Some(now) = nt.pop() {
        let v = &balls[now];
        let (a, b) = (v[0], v[1]);
        for a in [a, b] {
            t[a].pop();
            if let Some(&last) = t[a].last() {
                balls[last].push(a);
                if balls[last].len() == 2 {
                    nt.push(last);
                }
            }
        }
    }

    if t.iter().all(|v| v.is_empty()) {
        println!("Yes")
    } else {
        println!("No")
    }
}

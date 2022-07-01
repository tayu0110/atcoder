use proconio::input;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m], q: usize, s: [(usize, usize); q]};

    let mut t = vec![vec![]; n];
    for (a, b) in p {
        t[a-1].push(b-1);
        t[b-1].push(a-1);
    }

    let b = ((2*m) as f64).sqrt() as usize;
    let mut large = vec![];
    for i in 0..n {
        if t[i].len() >= b {
            large.push(i);
        }
    }

    let mut link = vec![vec![false; large.len()]; n];
    for (i, v) in large.iter().enumerate() {
        for j in &t[*v] {
            link[*j][i] = true;
        }
        link[*v][i] = true;
    }


    let mut update_large = vec![-1i64; large.len()];
    let mut update = vec![-1i64; n];

    for (i, (x, y)) in s.iter().enumerate() {
        let (x, _) = (*x - 1, *y);
        let mut last = update[x];
        for j in 0..large.len() {
            if link[x][j] {
                last = std::cmp::max(last, update_large[j]);
            }
        }

        let res = if last == -1 { 1 } else { let (_, y) = s[last as usize]; y };
        println!("{}", res);
        if t[x].len() < b {
            update[x] = i as i64;
            for to in &t[x] {
                update[*to] = i as i64;
            }
        } else {
            for (j, v) in large.iter().enumerate() {
                if v == &x {
                    update_large[j] = i as i64;
                }
            }
        }
    }
}
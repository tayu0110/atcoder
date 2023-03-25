use proconio::input;

fn solve(r: usize, c: usize, now: usize, a: &Vec<Vec<usize>>, memo: &mut Vec<std::collections::HashMap<usize, usize>>) {
    let n = a.len();

    if r + c == n - 1 {
       *memo[c].entry(now).or_insert(0) += 1;
        return;
    }

    if r + 1 < n {
        solve(r+1, c, now ^ a[r][c], a, memo);
    }
    if c + 1 < n {
        solve(r, c+1, now ^ a[r][c], a, memo);
    }
}

fn main() {
    input! {n: usize, mut a: [[usize; n]; n]}

    let mut m1 = vec![std::collections::HashMap::new(); n];
    let mut m2 = m1.clone();

    solve(0, 0, 0, &a, &mut m1);

    for v in a.iter_mut() {
        v.reverse();
    }
    a.reverse();

    solve(0, 0, 0, &a, &mut m2);
    m2.reverse();

    let mut res = 0;
    for i in 0..n {
        let (r, c) = (i, n-1-i);

        let na = a[r][c];

        for (k, v) in &m1[i] {
            let t = *k ^ na;

            if let Some(w) = m2[i].get(&t) {
                res += w * *v;
            }
        }
    }

    println!("{}", res);
}
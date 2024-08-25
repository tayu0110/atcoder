use proconio::*;

fn main() {
    input! {n: usize, t: usize, a: [usize; t]}

    let mut map = vec![vec![usize::MAX; n]; n];
    for (i, a) in a.into_iter().enumerate() {
        let r = (a - 1) / n;
        let c = (a - 1) % n;
        map[r][c] = map[r][c].min(i + 1);
    }

    let mut res = usize::MAX;
    for i in 0..n {
        res = res.min(*map[i].iter().max().unwrap());
        res = res.min((0..n).map(|j| map[j][i]).max().unwrap());
    }

    res = res.min({
        let mut max = 0;
        for i in 0..n {
            max = max.max(map[i][i]);
        }
        max
    });

    res = res.min({
        let mut max = 0;
        for i in 0..n {
            max = max.max(map[i][n - 1 - i]);
        }
        max
    });

    println!("{}", res as i64);
}

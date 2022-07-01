use proconio::input;

const INF: usize = 1001001001001001001;

fn dfs(now: usize, res: &mut Vec<(usize, usize)>, t: &Vec<Vec<(usize, usize)>>) -> bool {
    let mut k = 0;
    for (to, idx) in &t[now] {
        if res[*idx] != (INF, INF) {
            continue;
        }
        res[*idx] = (now, *to);
        let f = dfs(*to, res, t);
        if !f {
            res[*idx] = (*to, now);
        } else {
            k += 1;
        }
    }

    k % 2 == 0
}

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m]};

    if m % 2 == 1 {
        println!("-1");
        std::process::exit(0);
    }

    let mut t = vec![vec![]; n];
    for (i, (a, b)) in p.iter().enumerate() {
        t[*a-1].push((*b-1, i));
        t[*b-1].push((*a-1, i));
    }

    let mut res = vec![(INF, INF); m];
    dfs(0, &mut res, &t);
    for (c, d) in res {
        println!("{} {}", c+1, d+1);
    }
}
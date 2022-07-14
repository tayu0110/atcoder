use proconio::input;

const INF: usize = 0x3f3f3f3f3f3f3f3f;

fn dfs(now: usize, num: &mut i32, ck: &mut Vec<i32>, t: &Vec<Vec<usize>>) {
    ck[now] = *num;
    for to in &t[now] {
        if ck[*to] >= 0 {
            continue;
        }

        *num += 1;
        dfs(*to, num, ck, t);
    }
}

fn doubling(now: usize, par: usize, depth: &mut Vec<usize>, d: &mut Vec<Vec<usize>>, t: &Vec<Vec<usize>>) {
    d[0][now] = par;

    let mut nt = par;
    let mut cnt = 1;
    while nt != INF {
        d[cnt][now] = d[cnt-1][nt];
        nt = d[cnt-1][nt];
        cnt += 1;
    }

    for to in &t[now] {
        if to != &par {
            depth[*to] = depth[now] + 1;
            doubling(*to, now, depth, d, t);
        }
    }
}

fn main() {
    input! {n: usize, p: [(usize, usize); n-1], q: usize};

    let mut t = vec![vec![]; n];
    for (a, b) in p {
        t[a-1].push(b-1);
        t[b-1].push(a-1);
    }

    let mut ck = vec![-1; n];
    dfs(0, &mut 0, &mut ck, &t);


    let mut d = vec![vec![INF; n]; 30];
    let mut depth = vec![0; n];
    doubling(0, INF, &mut depth, &mut d, &t);

    let mut ans = vec![];

    for _ in 0..q {
        input! {k: usize, v: [usize; k]};

        let mut w = vec![];
        for now in v {
            w.push((ck[now-1], now-1));
        }
        w.sort();
        w.push(w[0]);

        let mut res = 0;
        for i in (0..k+1).skip(1) {
            let (_, mut l) = w[i-1];
            let (_, mut r) = w[i];
            if depth[l] > depth[r] {
                std::mem::swap(&mut l, &mut r);
            }

            res += depth[r] - depth[l];
            let mut cnt = 30;
            while depth[r] > depth[l] {
                cnt -= 1;
                if depth[r] - depth[l] < 1 << cnt {
                    continue;
                }
                r = d[cnt][r];
            }

            let mut cnt = 30;
            while l != r && cnt > 0 {
                cnt -= 1;
                if d[cnt][l] == INF {
                    continue;
                }
                if d[cnt][l] != d[cnt][r] {
                    res += 2 << cnt;
                    l = d[cnt][l];
                    r = d[cnt][r];
                }
            }
            
            if l != r {
                res += 2;
            }
        }

        ans.push(res);
    }

    for v in ans {
        println!("{}", v / 2);
    }
}
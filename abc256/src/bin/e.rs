use proconio::input;

mod scc {
    #[allow(dead_code)]
    fn dfs(now: usize, g: i32, stack: &mut Vec<usize>, group: &mut Vec<i32>, t: &Vec<Vec<usize>>) {
        group[now] = g;
        for to in &t[now] {
            if group[*to] < 0 {
                dfs(*to, g, stack, group, t);
            }
        }
        stack.push(now);
    }
    #[allow(dead_code)]    
    fn dfs2(now: usize, g: i32, group: &Vec<i32>, s: &mut Vec<i32>, r: &Vec<Vec<usize>>) {
        s[now] = g;
        for to in &r[now] {
            if group[now] == group[*to] && s[*to] < 0 {
                dfs2(*to, g, group, s, r);
            }
        }
    }
    #[allow(dead_code)]
    // Return the array of each vertex with ID for each strongly connected component
    pub fn scc(n: usize, edges: &Vec<(usize, usize)>) -> Vec<i32> {
        let mut t = vec![vec![]; n];
        let mut rt = vec![vec![]; n];
        for (a, b) in edges {
            t[*a].push(*b);
            rt[*b].push(*a);
        }
        let mut group = vec![-1; n];
        let mut v = vec![];
        let mut g = 0;
        for i in 0..n {
            if group[i] < 0 {
                let mut stack = vec![];
                dfs(i, g, &mut stack, &mut group, &t);
                g += 1;
                v.push(stack);
            }
        }
        let mut g = 0;
        let mut res = vec![-1; n];
        for w in v.iter_mut() {
            while let Some(now) = w.pop() {
                
                if res[now] < 0 {
                    dfs2(now, g, &group, &mut res, &rt);
                    g += 1;
                }
            }
        }
        res
    }
}

fn main() {
    input! {n: usize, x: [usize; n], c: [usize; n]};

    let edges = x.iter().enumerate().map(|(i, x)| (i, *x-1)).collect::<Vec<(usize, usize)>>();
    let t = scc::scc(n, &edges);

    let mut v = vec![vec![]; n];
    for (i, w) in t.iter().enumerate() {
        v[*w as usize].push(i);
    }

    const INF: usize = 111222333444555666;
    let mut ans = 0;
    for w in v {
        if w.len() < 2 {
            continue;
        }
        let mut res = INF;
        for s in w {
            res = std::cmp::min(res, c[s]);
        }
        ans += res;
    }

    println!("{}", ans);
}
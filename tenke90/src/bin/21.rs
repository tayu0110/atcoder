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
            t[*a-1].push(*b-1);
            rt[*b-1].push(*a-1);
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
    input! {n: usize, m: usize, p: [(usize, usize); m]};

    let s = scc::scc(n, &p);

    let mut res = vec![0usize; n];
    for v in s {
        res[v as usize] += 1;
    }

    println!("{}", res.iter().fold(0, |sum, x| if *x > 0 { sum + *x * (*x-1) / 2 } else { sum }));
}
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
            while !w.is_empty() {
                let now = w.pop().unwrap();
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

    let buf = scc::scc(n, &p);

    let mut res = vec![vec![]; n];
    for (i, v) in buf.iter().enumerate() {
        res[*v as usize].push(i);
    }

    while !res.is_empty() {
        if res.last().unwrap().len() > 0 {
            break;
        }
        res.pop().unwrap();
    }

    println!("{}", res.len());
    for v in res {
        print!("{}", v.len());
        for w in v {
            print!(" {}", w);
        }
        println!("");
    }
}
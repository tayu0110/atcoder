use proconio::input;

fn dfs(n: usize, k: usize, depth: usize, ie: &mut Vec<i32>, nt: &mut Vec<usize>, tmp: &mut Vec<usize>, res: &mut Vec<Vec<usize>>, t: &Vec<Vec<usize>>) -> bool {
    if n == depth {
        res.push(tmp.clone());
        return true;
    }

    if nt.is_empty() {
        return false;
    }

    for i in (0..nt.len()).rev() {
        if res.len() == k {
            break;
        }

        let x = nt[i];
        nt.remove(i);
        for to in &t[x] {
            ie[*to] -= 1;
            if ie[*to] == 0 {
                nt.push(*to);
            }
        }
        tmp[depth] = x;
        let r = dfs(n, k, depth+1, ie, nt, tmp, res, t);
        if !r {
            return false;
        }
        for to in &t[x] {
            if ie[*to] == 0 {
                nt.pop();
            }
            ie[*to] += 1;
        }
        nt.insert(i, x);
    }
    true
}

fn main() {
    input! {n: usize, m: usize, k: usize, p: [(usize, usize); m]};

    let mut t = vec![vec![]; n];
    let mut ie = vec![0; n];
    for (a, b) in p {
        t[a-1].push(b-1);
        ie[b-1] += 1;
    }

    let mut nt = vec![];
    let mut tmp = vec![0; n];
    let mut res = vec![];
    for i in 0..n {
        if ie[i] == 0 {
            nt.push(i);
        }
    }

    dfs(n, k, 0, &mut ie, &mut nt, &mut tmp, &mut res, &t);
    if res.len() != k {
        println!("-1");
    } else {
        for v in res {
            for i in 0..v.len() {
                if i > 0 {
                    print!(" ");
                }
                print!("{}", v[i]+1);
            }
            println!();
        }
    }
}
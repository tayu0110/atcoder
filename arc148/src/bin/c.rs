#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, q: usize, mut p: [usize; n-1]};
    p.iter_mut().for_each(|v| { *v -= 1; });
    p.insert(0, 0);

    let mut t = vec![vec![]; n];
    for i in (0..n).skip(1) {
        let par = p[i];
        t[par].push(i);
    }
    
    let mut ans = vec![];
    for _ in 0..q {
        input! {m: usize, mut v: [usize; m]};
        v.iter_mut().for_each(|v| { *v -= 1; });
        v.sort();
        
        let mut map = std::collections::HashMap::new();
        for (i, now) in v.iter().enumerate() {
            map.insert(*now, i);
        }
            
        let mut nt = vec![vec![]; m];
        for (i, now) in v.iter().enumerate() {
            if *now == 0 {
                continue;
            }
            let par = p[*now];
            if let Some(mpar) = map.get(&par) {
                nt[*mpar].push(i);
            }
        }

        let mut back = vec![false; m];
        let mut res = 0;
        for i in 0..m {
            if !back[i] {
                back[i] = true;
                res += 1;
            }

            for to in &nt[i] {
                back[*to] = true;
            }

            res += t[v[i]].len() - nt[i].len();
        }

        ans.push(res);
    }

    for v in ans {
        println!("{}", v);
    }
}

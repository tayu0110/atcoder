use proconio::*;

fn main() {
    input! {n: usize, m: usize, mut p: [usize; n-1], e: [(usize, usize); m]}
    let mut children = vec![0; n];
    for &p in &p {
        children[p] += 1;
    }
    p.insert(0, 0);

    let mut edge = vec![vec![]; n];
    let mut nt = vec![];
    for (b, c) in e {
        nt.push(b);
        edge[b].push(c);
    }

    let mut res = 0;
    while !nt.is_empty() {
        let mut new = vec![];
        for n in nt {
            if n == 0 {
                res += edge[n].iter().sum::<usize>();
                break;
            }
            let &min = edge[n].iter().min().unwrap();
            let sum = edge[n].iter().map(|&v| v - min).sum::<usize>();
            // eprintln!("n: {}, sum: {}, min: {}", n, sum, min);
            res += sum;
            edge[p[n]].push(min);
            children[p[n]] -= 1;

            if children[p[n]] == 0 {
                new.push(p[n]);
            }
        }

        nt = new;
    }

    println!("{}", res)
}

#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, p: [usize; 2*n]};
    let mut p = p.into_iter().map(|v| v-1).collect::<Vec<_>>();

    if n == 1 {
        if p[0] > p[1] {
            println!("1");
            println!("1");
        } else {
            println!("0");
            println!();
        }
        std::process::exit(0);
    }

    let mut idx = vec![0; 2*n];
    for (i, v) in p.iter().enumerate() {
        idx[*v] = i;
    }
    let mut res = vec![];
    for i in 0..2*n {
        let v = idx[i];
        if v % 2 != 0 {
            if p[v-1] > i {
                res.push(v);
                idx[p[v-1]] = v;
                p.swap(v-1, v);
            } else if v+1 != 2*n && p[v+1] > i {
                res.push(v+1);
                idx[p[v+1]] = v;
                p.swap(v, v+1);
            }
        }
    }

    println!("{}", res.len());
    for i in 0..res.len() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", res[i]);
    }
    println!();
}

use im_rc::HashMap;
#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
	input! {n: usize};
    let mut map = HashMap::new();
    for i in 0..n {
        input! {m: usize, p: [(usize, usize); m]};
        for &(np, e) in &p {
            map.entry(np).or_insert(vec![]).push((e, i));
        }
    }

    if n == 1 {
        println!("1");
        std::process::exit(0);
    }

    let mut f = vec![0; n];
    for (_, mut v) in map {
        v.sort();
        v.reverse();
        if v.len() == 1 {
            f[v[0].1] = 1;
        } else {
            if v[0].0 == v[1].0 {
                continue;
            }
            f[v[0].1] = 1;
        }
    }

    let mut res = f.into_iter().sum::<usize>();
    if res != n {
        res += 1;
    }

    println!("{}", res);
}

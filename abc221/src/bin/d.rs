#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, p: [(usize, usize); n]}

    let p = p.into_iter().map(|(a, b)| (a, a+b)).collect::<Vec<_>>();

    let mut comp = std::collections::BTreeMap::new();
    for (a, b) in &p {
        comp.insert(*a, 0);
        comp.insert(*b, 0);
    }

    let mut rcomp = std::collections::HashMap::new();
    let mut cnt = 0;
    for (k, v) in comp.iter_mut() {
        *v = cnt;
        rcomp.insert(cnt, *k);
        cnt += 1;
    }

    let mut d = vec![0i32; cnt+1];
    for (a, b) in p {
        d[*comp.get(&a).unwrap()] += 1;
        d[*comp.get(&b).unwrap()] -= 1;
    }

    for i in 0..cnt-1 {
        d[i+1] += d[i];
    }

    let mut res = vec![0; n+1];
    for i in 0..cnt-1 {
        let (f, s) = (rcomp.get(&i).unwrap(), rcomp.get(&(i+1)).unwrap());

        res[d[i] as usize] += *s - *f;
    }

    for i in 1..=n {
        if i > 1 {
            print!(" ");
        }
        print!("{}", res[i]);
    }
    println!("");
}

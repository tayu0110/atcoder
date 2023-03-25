#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
	input! {n: usize, s: [String; n]};

    // let mut map = std::collections::HashMap::new();
    // for v in s {
    //     if !map.contains_key(&v) {
    //         println!("{}", v);
    //         map.insert(v, 1);
    //     } else {
    //         let x = map.get(&v).unwrap();
    //         println!("{}({})", v, x);
    //         *map.entry(v).or_default() += 1;
    //     }
    // }
    let mut list = s.into_iter().enumerate().map(|(i, s)| (s, i)).collect::<Vec<_>>();
    list.sort();
    let mut buf = vec![];
    for (s, i) in list {
        let (ni, ns, nres) = match buf.last() {
            Some((_, ps, res)) if *ps == s => (i, s, res+1),
            _ => (i, s, 0)
        };
        buf.push((ni, ns, nres));
    }

    buf.sort();
    for (_, s, res) in buf {
        if res == 0 {
            println!("{}", s);
        } else {
            println!("{}({})", s, res);
        }
    }
}

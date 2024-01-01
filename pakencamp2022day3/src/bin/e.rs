use itertools::Itertools;
use std::collections::HashSet;

const LEN: usize = 100000;
const MAX: usize = 2000_000;

fn verify(a: &[usize], b: &[usize]) {
    eprintln!("start verify");
    let mut set = HashSet::new();
    for a in a {
        for b in b {
            if a * b > MAX {
                break;
            }
            set.insert(a * b);
        }
    }
    assert_eq!(set.len(), MAX);
}

fn main() {
    let mut t = vec![vec![]; MAX + 1];
    let mut a = vec![];
    let mut b = vec![];

    for i in 1..=MAX {
        let mut max = 1;
        for j in (1..).take_while(|&j| j * j <= i) {
            if i % j == 0 {
                max = j;
            }
        }

        t[max].push(i / max);
    }

    for i in 1..=MAX {
        for (k, &j) in t[i].iter().enumerate() {
            if k % 2 == 0 {
                a.push(i);
                b.push(j);
            } else {
                a.push(j);
                b.push(i);
            }
        }
    }

    a.sort();
    a.dedup();
    b.sort();
    b.dedup();

    eprintln!("a: {}", a.len());
    eprintln!("b: {}", b.len());

    assert!(a.len() <= LEN);
    assert!(b.len() <= LEN);

    a.resize(LEN, MAX);
    b.resize(LEN, MAX);

    if cfg!(debug_assertions) {
        verify(&a, &b);
    }

    println!("{}", a.iter().join(" "));
    println!("{}", b.iter().join(" "));
}

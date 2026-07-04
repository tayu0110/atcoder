use itertools::Itertools;
use proconio::*;

fn main() {
    input! {h: usize, _w: usize, mut c: [marker::Bytes; h]}

    while c[0].iter().all(|&c| c == b'.') {
        c.remove(0);
    }
    while c.last().is_some_and(|c| c.iter().all(|&c| c == b'.')) {
        c.pop();
    }
    while c.iter().all(|c| c[0] == b'.') {
        c.iter_mut().for_each(|c| {
            c.remove(0);
        });
    }
    while c.iter().all(|c| c.last() == Some(&b'.')) {
        c.iter_mut().for_each(|c| {
            c.pop();
        });
    }
    for c in c {
        println!("{}", c.iter().map(|c| *c as char).join(""))
    }
}

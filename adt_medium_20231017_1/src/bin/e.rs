use itertools::Itertools;
use proconio::*;
use std::collections::HashSet;

fn find(
    now: &mut Vec<String>,
    mut len: usize,
    ns: &[String],
    t: &HashSet<String>,
) -> Option<String> {
    if len > 16 {
        return None;
    }

    now.push(ns.last().unwrap().clone());
    len += ns.last().unwrap().len();

    if ns.len() == 1 {
        if !(3..=16).contains(&len) {
            now.pop();
            return None;
        }
        let s = now
            .iter()
            .fold(String::new(), |s, v| [s, v.clone()].concat());
        if t.contains(&s) {
            now.pop();
            return None;
        } else {
            return Some(s);
        }
    }
    let mut cnt = 0;
    for _ in 0..16 {
        now.push("_".to_owned());
        cnt += 1;
        len += 1;
        if let Some(res) = find(now, len, &ns[..ns.len() - 1], t) {
            return Some(res);
        }

        if len > 16 {
            break;
        }
    }

    for _ in 0..cnt {
        now.pop();
    }
    now.pop();

    None
}

fn main() {
    input! {n: usize, m: usize, s: [String; n], t: [String; m]}

    let t = t.into_iter().collect::<HashSet<_>>();

    for v in (0..n).permutations(n) {
        let mut ns = vec![];
        for i in v {
            ns.push(s[i].clone());
        }

        let mut buf = vec![];
        if let Some(res) = find(&mut buf, 0, &ns, &t) {
            println!("{res}");
            return;
        }
    }

    println!("-1")
}

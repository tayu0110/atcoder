use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

const NULL: char = '\0';

fn main() {
    input! {s: Chars, t: Chars, u: Chars}

    let mut v = s
        .iter()
        .chain(t.iter().chain(u.iter()))
        .map(|&c| c)
        .collect::<Vec<char>>();
    v.sort();
    v.dedup();

    if v.len() > 10 {
        println!("UNSOLVABLE");
        return;
    }

    while v.len() < 10 {
        v.push(NULL);
    }
    v.sort();

    for v in v.into_iter().permutations(10) {
        let ns = s
            .iter()
            .map(|&c| v.iter().position(|&d| c == d).unwrap())
            .fold(0, |s, v| s * 10 + v);
        let nt = t
            .iter()
            .map(|&c| v.iter().position(|&d| c == d).unwrap())
            .fold(0, |s, v| s * 10 + v);
        let nu = u
            .iter()
            .map(|&c| v.iter().position(|&d| c == d).unwrap())
            .fold(0, |s, v| s * 10 + v);

        if ns == 0 || nt == 0 || nu == 0 {
            continue;
        }

        if ns.to_string().len() < s.len()
            || nt.to_string().len() < t.len()
            || nu.to_string().len() < u.len()
        {
            continue;
        }

        if ns + nt == nu {
            println!("{}", ns);
            println!("{}", nt);
            println!("{}", nu);
            return;
        }
    }

    println!("UNSOLVABLE");
}

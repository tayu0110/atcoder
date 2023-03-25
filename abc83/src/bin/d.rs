use proconio::*;

fn main() {
    input! {s: marker::Chars}

    let n = s.len();
    let mut t = vec![];
    for c in s {
        match t.last_mut() {
            Some((pc, cnt)) if c == *pc => *cnt += 1,
            _ => t.push((c, 1)),
        }
    }

    if t.len() < 3 {
        println!("{}", t.into_iter().map(|(_, cnt)| cnt).max().unwrap());
        return;
    }

    let t = t.into_iter().map(|(_, cnt)| cnt).collect::<Vec<_>>();
    let mut res = std::usize::MAX;
    let mut now = 0;
    for c in t {
        res = res.min(now.max(n - now));
        now += c;
    }

    println!("{}", res)
}

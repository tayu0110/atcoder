use proconio::*;

fn main() {
    input! {_: usize, k: usize, s: marker::Chars}

    let mut ns = vec![];
    for c in s {
        match ns.last_mut() {
            Some((pc, cnt)) if pc == &c => *cnt += 1,
            _ => ns.push((c, 1)),
        }
    }

    let mut res = 0;
    let mut now = 0;
    let mut nk = 0;
    let (mut l, mut r) = (0, 0);
    while l < ns.len() {
        while r < ns.len() {
            if nk < k {
                now += ns[r].1;
                if ns[r].0 == '0' {
                    nk += 1;
                }
            } else {
                if ns[r].0 == '0' {
                    break;
                }

                now += ns[r].1;
            }

            r += 1;
        }

        res = std::cmp::max(res, now);

        if ns[l].0 == '0' {
            nk -= 1;
        }
        now -= ns[l].1;

        l += 1;
    }

    println!("{}", res);
}

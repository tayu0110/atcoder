use proconio::*;

fn rec(
    _l: usize,
    index: usize,
    _now: usize,
    eq: bool,
    v: &[usize],
    res: &mut Vec<(usize, usize, usize)>,
) {
    if index == v.len() {
        return;
    }

    if eq {
        for w in 0..=v[index] {
            if w == v[index] {
                res.push((
                    2 * index + 1,
                    2 * index + 3,
                    w * 5usize.pow((v.len() - index - 1) as u32),
                ));
                rec(
                    _l,
                    index + 1,
                    _now + w * 5usize.pow((v.len() - index - 1) as u32),
                    true,
                    v,
                    res,
                );
            } else {
                res.push((
                    2 * index + 1,
                    2 * index + 2,
                    w * 5usize.pow((v.len() - index - 1) as u32),
                ));
                rec(
                    _l,
                    index + 1,
                    _now + w * 5usize.pow((v.len() - index - 1) as u32),
                    false,
                    v,
                    res,
                );
            }
        }
    } else {
        for w in 0..5 {
            res.push((
                2 * index,
                2 * index + 2,
                w * 5usize.pow((v.len() - index - 1) as u32),
            ));
            rec(
                _l,
                index + 1,
                _now + w * 5usize.pow((v.len() - index - 1) as u32),
                false,
                v,
                res,
            );
        }
    }
}

fn main() {
    input! {l: usize}

    let mut r = l;
    let mut v = vec![];
    while r > 0 {
        v.push(r % 5);
        r /= 5;
    }
    v.reverse();

    let mut res = vec![];
    rec(l, 0, 0, true, &v, &mut res);

    res.sort();
    res.dedup();

    let n = res.iter().map(|&(l, r, _)| l.max(r)).max().unwrap();
    let m = res.len();

    println!("{} {}", n, m);
    for (u, v, c) in res {
        println!("{} {} {}", u, v, c);
    }
}

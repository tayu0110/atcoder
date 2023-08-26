use proconio::*;

fn main() {
    input! {n: usize, m: usize, q: usize, e: [(usize, usize, usize); m], q: [(usize, usize, usize, usize); q]}

    let mut t = vec![vec![]; n];
    for (a, b, c) in e {
        t[a - 1].push((b, c));
    }

    let mut terms: Vec<(usize, usize)> = vec![];
    for i in 0..n {
        if t[i].is_empty() {
            continue;
        }

        t[i].sort();

        let mut new = vec![];
        for &(b, c) in &t[i] {
            match new.last_mut() {
                Some((_, pc)) if b <= *pc => *pc = c.max(*pc),
                _ => new.push((b, c)),
            }
        }

        terms.extend(new.iter());
        t[i] = new;
    }

    terms.sort();
    let terms = {
        let mut buf = vec![];
        for (b, c) in terms {
            if buf.is_empty() {
                buf.push((b, c));
                continue;
            }

            let len = buf.len();
            if c <= buf[len - 1].1 {
                continue;
            }

            if len >= 2 && b <= buf[len - 2].1 {
                buf.pop();
            }

            buf.push((b, c));
        }
        buf
    };

    // eprintln!("terms: {terms:?}");

    let upper_bound = |idx: usize| -> Option<usize> {
        let (mut l, mut r) = (-1, terms.len() as i32);
        while r - l > 1 {
            let m = (r + l) / 2;
            let (s, t) = terms[m as usize];
            if s <= idx && idx <= t {
                l = m;
            } else if idx < s {
                r = m;
            } else {
                l = m;
            }
        }

        if r < terms.len() as i32 {
            let (s, t) = terms[r as usize];
            if s <= idx && idx <= t {
                return Some(r as usize);
            }
        }

        if l >= 0 {
            let (s, t) = terms[l as usize];
            if s <= idx && idx <= t {
                return Some(l as usize);
            }
        }

        None
    };

    let lower_bound = |idx: usize| -> Option<usize> {
        let (mut l, mut r) = (-1, terms.len() as i32);
        while r - l > 1 {
            let m = (r + l) / 2;
            let (s, t) = terms[m as usize];
            if s <= idx && idx <= t {
                r = m;
            } else if idx < s {
                r = m;
            } else {
                l = m;
            }
        }

        // eprintln!("l: {l}, r: {r}");

        if l >= 0 {
            let (s, t) = terms[l as usize];
            if s <= idx && idx <= t {
                return Some(l as usize);
            }
        }

        if r < terms.len() as i32 {
            let (s, t) = terms[r as usize];
            if s <= idx && idx <= t {
                return Some(r as usize);
            }
        }

        None
    };

    'base: for (x, y, z, w) in q.into_iter().map(|(x, y, z, w)| (x - 1, y, z - 1, w)) {
        if y == w {
            if x == z {
                println!("0");
            } else {
                println!("1");
            }
            continue;
        }

        if !t[x].is_empty() {
            let mut res = 0;
            let (mut l, mut r) = (-1, t[x].len() as i32);
            while r - l > 1 {
                let m = ((r + l) / 2) as usize;
                if t[x][m].1 < y {
                    l = m as i32;
                } else if y < t[x][m].0 {
                    r = m as i32;
                } else {
                    let (s, t) = t[x][m];
                    if y < w {
                        if w <= t {
                            res += w - y;
                            if x != z {
                                res += 1;
                            }
                            println!("{res}");
                            continue 'base;
                        } else {
                            break;
                        }
                    } else {
                        if s <= w {
                            res += y - w;
                            if x != z {
                                res += 1;
                                println!("{res}");
                                continue 'base;
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        if y < w {
            // eprintln!("upy: {:?}, low: {:?}", upper_bound(y), lower_bound(w));
            if let (Some(yi), Some(wi)) = (upper_bound(y), lower_bound(w)) {
                println!("{}", w - y + wi - yi);
            } else {
                println!("-1");
            }
        } else {
            // eprintln!("upw: {:?}, loy: {:?}", upper_bound(w), lower_bound(y));
            if let (Some(wi), Some(yi)) = (upper_bound(w), lower_bound(y)) {
                println!("{}", y - w + yi - wi);
            } else {
                println!("-1")
            }
        }
    }
}

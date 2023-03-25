use itertools::Itertools;
use proconio::input;

fn main() {
    input! {n: usize, k: usize, p: [usize; n]}

    if k == 0 {
        println!("{}", p.iter().join(" "));
        return;
    }

    let mut a = p
        .iter()
        .enumerate()
        .map(|(i, v)| (*v, i))
        .collect::<Vec<_>>();
    a.sort();

    for (_, pos) in a {
        if pos > k && n - pos > k {
            continue;
        }

        let f = (pos <= k).then(|| {
            let mut k = k - pos;
            let mut nt = vec![];
            for &f in &p[pos..] {
                if k == 0 {
                    nt.push(f);
                    continue;
                }
                while let Some(back) = nt.pop() {
                    if back < f {
                        nt.push(back);
                        break;
                    }
                    k -= 1;
                    if k == 0 {
                        break;
                    }
                }
                nt.push(f);
            }
            nt[0..nt.len() - k].to_vec()
        });
        let b = (n - pos <= k).then(|| {
            let mut k = k - (n - pos);
            let mut nt = vec![];
            for &b in &p[pos..] {
                while let Some((back, _)) = nt.pop() {
                    if back < b {
                        nt.push((back, true));
                        break;
                    }
                }
                nt.push((b, true));
            }
            for &b in &p[..pos] {
                while let Some((back, f)) = nt.pop() {
                    if back < b {
                        nt.push((back, f));
                        break;
                    }
                    if k == 0 && !f {
                        nt.push((back, f));
                        break;
                    }
                    if !f {
                        k -= 1;
                    }
                }
                nt.push((b, false));
            }
            nt.iter()
                .take(nt.len() - k)
                .map(|&(b, _)| b)
                .collect::<Vec<_>>()
        });

        if let (Some(f), Some(b)) = (&f, &b) {
            if let Some((fi, ba)) = f.iter().zip(b.iter()).skip_while(|(f, b)| f == b).next() {
                if fi < ba {
                    println!("{}", f.iter().join(" "))
                } else {
                    println!("{}", b.iter().join(" "))
                }
            } else {
                if f.len() < b.len() {
                    println!("{}", f.iter().join(" "))
                } else {
                    println!("{}", b.iter().join(" "))
                }
            }
        } else if let Some(f) = f {
            println!("{}", f.iter().join(" "))
        } else if let Some(b) = b {
            println!("{}", b.iter().join(" "))
        }

        return;
    }

    unreachable!()
}

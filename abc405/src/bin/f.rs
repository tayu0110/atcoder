use proconio::*;

fn normalize(n: usize, offset: usize, p: &mut [(usize, usize)]) {
    for (a, b) in p {
        *a = (*a + offset) % n;
        *b = (*b + offset) % n;
        if *a > *b {
            std::mem::swap(a, b);
        }
    }
}

fn main() {
    input! {n: usize, m: usize, mut p: [(usize, usize); m], q: usize, mut query: [(usize, usize); q]}
    p.iter_mut().for_each(|p| {
        p.0 -= 1;
        p.1 -= 1;
    });
    query.iter_mut().for_each(|q| {
        q.0 -= 1;
        q.1 -= 1;
    });

    p.sort_unstable_by_key(|&(a, b)| {
        let diff = a.abs_diff(b);
        diff.min(n * 2 - diff)
    });
    let offset = {
        let (a, b) = p[0];
        let diff = a.abs_diff(b);
        if diff < n {
            n * 2 - a.min(b)
        } else {
            n * 2 - a.max(b)
        }
    };
    normalize(n * 2, offset, &mut p);
    normalize(n * 2, offset, &mut query);

    p.sort_unstable_by_key(|&(a, b)| if a == 0 { 0 } else { a + (n * 2 - b) });
    eprintln!("p: {p:?}");

    let mut cell = vec![usize::MAX; 2 * n];
    for (i, (a, b)) in p.into_iter().enumerate() {
        if a == 0 {
            for j in 0..=b {
                cell[j] = i;
            }
        } else {
            for j in (0..=a).rev() {
                if cell[j] != usize::MAX {
                    break;
                }
                cell[j] = i;
            }
            for j in b..n * 2 {
                if cell[j] != usize::MAX {
                    break;
                }
                cell[j] = i;
            }
        }
    }

    for i in 0..n * 2 {
        if cell[i] == usize::MAX {
            cell[i] = m;
        }
    }
    eprintln!("cell: {cell:?}");

    for (c, d) in query {
        println!("{}", cell[c].abs_diff(cell[d]));
    }
}

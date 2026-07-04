use itertools::Itertools;
use math::MathInt;
use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n], q: usize, query: [(u32, u32, u32); q]}

    let block = q.sqrti() * 2;

    let mut cum = vec![];
    for query in query.chunks(block) {
        let mut c = vec![0usize; n + 1];
        for &(l, r, k) in query {
            c[l as usize - 1] = c[l as usize - 1].wrapping_add(k as usize);
            c[r as usize] = c[r as usize].wrapping_sub(k as usize);
        }
        for i in 0..n {
            c[i + 1] = c[i + 1].wrapping_add(c[i]);
        }
        cum.push(c);
    }

    for i in 0..n {
        for j in 0..cum.len() - 1 {
            cum[j + 1][i] += cum[j][i];
        }
    }
    // eprintln!("block: {block}, cum: {cum:?}");

    let mut ft = vec![vec![0u32; n + 1]; cum.len()];
    let mut ret = vec![0usize; q];
    for i in 0..n {
        if cum.last().unwrap()[i] <= a[i] {
            continue;
        }

        let pos = cum.partition_point(|cum| cum[i] <= a[i]);
        let mut base = 0;
        if pos > 0 {
            base += cum[pos - 1][i];
        }
        // eprintln!("i: {i}, pos: {pos}, base: {base}");
        let nth = pos;
        let chunk = &query[block * nth..(block * (nth + 1)).min(q)];
        for (j, &(l, r, k)) in chunk.iter().enumerate() {
            if (l - 1..r).contains(&(i as u32)) {
                if base >= a[i] {
                    // eprintln!("j: {}, sub: {k}", nth * block + j);
                    ret[nth * block + j] = ret[nth * block + j].wrapping_sub(k as usize);
                } else if base + k as usize >= a[i] {
                    // eprintln!("j: {}, sub: {}", nth * block + j, base + k - a[i]);
                    ret[nth * block + j] =
                        ret[nth * block + j].wrapping_sub(base + k as usize - a[i]);
                }
                base += k as usize;
            }
        }

        for j in pos + 1..ft.len() {
            ft[j][i + 1] += 1;
        }
    }
    for i in 0..ft.len() {
        for j in 0..n {
            ft[i][j + 1] += ft[i][j];
        }
    }

    for (i, (l, r, k)) in query.into_iter().enumerate() {
        ret[i] = ret[i].wrapping_add((r + 1 - l) as usize * k as usize);
        ret[i] = ret[i].wrapping_sub(
            (ft[i / block][r as usize] - ft[i / block][l as usize - 1]) as usize * k as usize,
        );
    }

    println!("{}", ret.iter().join("\n"))
}

use itertools::Itertools;
use proconio::*;
use utility::BinaryGrid;

fn main() {
    input! {s: [marker::Chars; 4], t: [marker::Chars; 4], u: [marker::Chars; 4]}

    let mut s: BinaryGrid = BinaryGrid::try_from(s).unwrap().truncate();
    let mut t: BinaryGrid = BinaryGrid::try_from(t).unwrap().truncate();
    let mut u: BinaryGrid = BinaryGrid::try_from(u).unwrap().truncate();

    for _ in 0..4 {
        for _ in 0..4 {
            for _ in 0..4 {
                {
                    let p = vec![&s, &t, &u];
                    for v in (0..3).permutations(3) {
                        let (s, t, u) = (p[v[0]], p[v[1]], p[v[2]]);
                        for i in 0..=4 {
                            for j in 0..=4 {
                                for k in 0..=4 {
                                    for l in 0..=4 {
                                        let mut buf = BinaryGrid::new(8, 8);
                                        buf.merge(0, 0, &s).ok();
                                        if buf.is_overflow(i, j, t)
                                            || buf.is_overlap(i, j, t).unwrap()
                                        {
                                            continue;
                                        }
                                        buf.merge(i, j, &t).ok();
                                        if buf.is_overflow(k, l, u)
                                            || buf.is_overlap(k, l, u).unwrap()
                                        {
                                            continue;
                                        }
                                        buf.merge(k, l, &u).ok();

                                        buf = buf.truncate();
                                        if buf.shape() == (4, 4)
                                            && buf.as_ref().into_iter().flatten().all(|&f| f)
                                        {
                                            println!("Yes");
                                            return;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                u = u.rotate90();
            }
            t = t.rotate90();
        }
        s = s.rotate90();
    }

    println!("No")
}

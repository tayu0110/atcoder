use proconio::*;

fn main() {
    input! {n: usize, _m: usize}

    let mut t = vec![];
    for _ in 0..n {
        input! {p: usize, c: usize, f: [usize; c]}
        t.push((p, c, f));
    }

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }

            let (p1, _c1, f1) = &t[i];
            let (p2, _c2, f2) = &t[j];

            let mut bad = false;
            bad |= p1 < p2;

            for &f1 in f1 {
                bad |= !f2.contains(&f1);
            }

            if bad {
                continue;
            }

            if p1 > p2 {
                println!("Yes");
                return;
            } else {
                for &f2 in f2 {
                    if !f1.contains(&f2) {
                        println!("Yes");
                        return;
                    }
                }
            }
        }
    }

    println!("No")
}

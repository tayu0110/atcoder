use proconio::*;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m]}

    let mut bad = vec![];
    for (a, b) in p {
        bad.push((a, b));
        for (da, db) in [(2usize, 1usize), (1, 2)] {
            for (da, db) in [
                (da, db),
                (da.wrapping_neg(), db),
                (da, db.wrapping_neg()),
                (da.wrapping_neg(), db.wrapping_neg()),
            ] {
                let na = a.wrapping_add(da);
                let nb = b.wrapping_add(db);

                if (1..=n).contains(&na) && (1..=n).contains(&nb) {
                    bad.push((na, nb));
                }
            }
        }
    }

    bad.sort_unstable();
    bad.dedup();
    println!("{}", n * n - bad.len());
}

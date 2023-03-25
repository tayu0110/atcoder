use proconio::input;

fn main() {
    input! {n: usize}

    let mut res = vec![];
    for a in 1..=9 {
        for b in 0..=9 {
            for c in 0..=9 {
                for x in 0..=9 {
                    for y in 0..=9 {
                        for z in 0..=9 {
                            res.push(
                                vec![a, a, x, y, b, b, c, z, c]
                                    .into_iter()
                                    .fold(0, |s, v| s * 10 + v),
                            );
                        }
                    }
                }
            }
        }
    }

    res.sort();
    res.dedup();

    println!("{}", res[n - 1]);
}

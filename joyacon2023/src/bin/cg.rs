use proconio::input;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut map = std::collections::HashMap::new();
    for a in a {
        *map.entry(a).or_insert(0) += 1;
    }

    let mut res = n * (n - 1) * (n - 2) / 6;
    for (_, v) in map {
        if v < 2 {
            continue;
        }

        res -= v * (v - 1) / 2 * (n - v);

        if v > 2 {
            res -= v * (v - 1) * (v - 2) / 6;
        }
    }

    println!("{}", res);
}

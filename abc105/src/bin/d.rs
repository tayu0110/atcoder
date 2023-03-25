use proconio::input;

fn main() {
    input! {n: usize, m: usize, a: [usize; n]}

    let mut v = vec![0; n + 1];
    let mut map = std::collections::HashMap::new();
    map.insert(0, 1usize);
    for i in 0..n {
        v[i + 1] = (v[i] + a[i]) % m;
        *map.entry(v[i + 1]).or_insert(0) += 1;
    }

    let mut res = 0;
    for (_, v) in map {
        res += v * (v - 1) / 2;
    }

    println!("{}", res)
}

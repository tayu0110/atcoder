use proconio::input;

fn main() {
    input! {n: usize, s: [String; n], m: usize, t: [String; m]}

    let mut map = std::collections::HashMap::new();
    for t in t {
        *map.entry(t).or_insert(0) += 1;
    }

    let mut ns = std::collections::HashMap::new();
    for s in s {
        *ns.entry(s).or_insert(0) += 1;
    }

    let mut res = 0;
    for (k, v) in ns {
        res = std::cmp::max(res, v - *map.get(&k).unwrap_or(&0));
    }

    println!("{}", res);
}

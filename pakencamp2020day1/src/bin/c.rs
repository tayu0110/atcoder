use proconio::input;

fn main() {
    input! {n: usize};

    let mut t = std::collections::HashMap::new();
    for _ in 0..n {
        input! {k: usize, s: [String; k]};
        for v in s {
            *t.entry(v).or_insert(0) += 1;
        }
    }

    println!("{}", t.iter().filter(|&(_, v)| v == &n).fold(0, |r, _| r + 1));
}

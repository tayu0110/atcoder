use proconio::input;

fn main() {
    input! {n: usize, m: usize, mut t: usize, a: [usize; n-1], p: [(usize, usize); m]}

    let mut map = std::collections::HashMap::new();
    for (x, y) in p {
        *map.entry(x - 1).or_insert(0) += y;
    }

    for i in 0..n - 1 {
        if let Some(rem) = map.get(&i) {
            t += *rem;
        }

        if a[i] >= t {
            println!("No");
            return;
        }

        t -= a[i];
    }

    println!("Yes");
}

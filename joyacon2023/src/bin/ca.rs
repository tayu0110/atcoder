use proconio::input;

fn main() {
    input! {n: usize, p: usize, q: usize, r: usize, a: [usize; n]}

    let mut b = vec![0];
    let mut set = std::collections::HashSet::new();
    for i in 0..n {
        b.push(b[i] + a[i]);
        set.insert(b[i + 1]);
    }

    for base in b.into_iter() {
        if set.contains(&(base + p))
            && set.contains(&(base + p + q))
            && set.contains(&(base + p + q + r))
        {
            println!("Yes");
            return;
        }
    }

    println!("No");
}

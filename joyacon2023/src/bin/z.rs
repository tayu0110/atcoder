use proconio::input;

fn main() {
    input! {n: usize, k: usize, a: [usize; n], b: [usize; k]}
    let b = b.into_iter().collect::<std::collections::HashSet<_>>();
    let max = *a.iter().max().unwrap();

    for i in 0..n {
        if a[i] == max && b.contains(&(i + 1)) {
            println!("Yes");
            return;
        }
    }

    println!("No")
}

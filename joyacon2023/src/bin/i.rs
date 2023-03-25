use proconio::input;

fn main() {
    input! {l: usize, r: usize, s: usize, t: usize}

    let mut a = vec![0; 110];
    for i in l..=r {
        a[i] |= 1;
    }
    for i in s..=t {
        a[i] |= 2;
    }

    println!(
        "{}",
        a.into_iter().filter(|j| *j == 3).count().saturating_sub(1)
    )
}

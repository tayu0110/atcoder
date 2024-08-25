use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [usize; n], b: [usize; m]}

    let mut c = a.clone();
    c.extend(b.iter());
    c.sort_unstable();

    for v in c.windows(2) {
        if a.contains(&v[0]) && a.contains(&v[1]) {
            println!("Yes");
            return;
        }
    }

    println!("No")
}

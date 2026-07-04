use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    if n <= 2 {
        println!("Yes");
        return;
    }

    for v in a.windows(2) {
        let (s, t) = (v[0], v[1]);
        let (a, b) = (a[0], a[1]);
        if s * b != t * a {
            println!("No");
            return;
        }
    }

    println!("Yes");
}

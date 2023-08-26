use proconio::*;

fn main() {
    input! {n: usize, m: usize, k: usize, mut e: usize, mut a: [usize; n], mut b: [usize; m]}

    a.sort();
    b.sort();
    b.reverse();
    let mut b = b[..k].to_vec();
    b.reverse();

    for i in 0..n {
        while e < a[i] {
            if let Some(b) = b.pop() {
                e += b;
            } else {
                println!("No");
                println!("{}", i);
                return;
            }
        }

        e -= a[i];
    }

    println!("Yes");
    println!("{}", k - b.len());
}

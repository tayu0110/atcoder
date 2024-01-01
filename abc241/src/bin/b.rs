use proconio::*;

fn main() {
    input! {n: usize, m: usize, mut a: [usize; n], b: [usize; m]}

    for b in b {
        let mut t = usize::MAX;
        for i in 0..a.len() {
            if a[i] == b {
                t = a.remove(i);
                break;
            }
        }

        if t == usize::MAX {
            println!("No");
            return;
        }
    }

    println!("Yes")
}

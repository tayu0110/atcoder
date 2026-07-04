use proconio::*;

fn main() {
    input! {a: [usize; 5]}

    for i in 0..4 {
        let mut a = a.clone();
        a.swap(i, i + 1);
        if a.windows(2).all(|v| v[0] < v[1]) {
            println!("Yes");
            return;
        }
    }
    println!("No")
}

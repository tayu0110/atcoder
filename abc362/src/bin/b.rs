use proconio::*;

fn main() {
    input! {a: [i32; 2], b: [i32; 2], c: [i32; 2]}

    let s = (a[0] - b[0]).pow(2) + (a[1] - b[1]).pow(2);
    let t = (a[0] - c[0]).pow(2) + (a[1] - c[1]).pow(2);
    let u = (b[0] - c[0]).pow(2) + (b[1] - c[1]).pow(2);

    let mut k = [s, t, u];
    k.sort();

    if k[0] + k[1] == k[2] {
        println!("Yes")
    } else {
        println!("No")
    }
}

use proconio::*;

fn main() {
    input! {n: usize, m: usize, x: i32, y: i32, xs: [i32; n], ys: [i32; m]}
    for z in x + 1..=y {
        if xs.iter().all(|x| *x < z) && ys.iter().all(|y| *y >= z) {
            println!("No War");
            return;
        }
    }

    println!("War")
}

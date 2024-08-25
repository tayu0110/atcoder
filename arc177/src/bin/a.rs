use proconio::*;

fn main() {
    input! {mut a: [usize; 6], n: usize, mut x: [usize; n]}
    a.reverse();

    for (mut a, c) in a.into_iter().zip([500, 100, 50, 10, 5, 1]) {
        for x in x.iter_mut() {
            let y = (*x / c).min(a);
            a -= y;
            *x -= y * c;
        }
    }

    if x.into_iter().all(|x| x == 0) {
        println!("Yes");
    } else {
        println!("No")
    }
}

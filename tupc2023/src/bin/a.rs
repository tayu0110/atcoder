use std::sync::Mutex;

use proconio::*;

const MAX: usize = 200010;

static X: Mutex<[usize; MAX]> = Mutex::new([0; MAX]);
static Y: Mutex<[usize; MAX]> = Mutex::new([0; MAX]);

fn main() {
    input! {n: usize, p: [(u32, u32, usize); n]}

    let mut x = X.lock().unwrap();
    let mut y = Y.lock().unwrap();
    for &(nx, ny, p) in &p {
        unsafe {
            *x.get_unchecked_mut(nx as usize) += p;
            *y.get_unchecked_mut(ny as usize) += p;
        }
    }

    let mut res = 0;
    for (nx, ny, p) in p {
        unsafe {
            res = res.max(*x.get_unchecked(nx as usize) + *y.get_unchecked(ny as usize) - p);
        }
    }

    println!("{res}")
}

use iolib::*;

fn main() {
    scan!(n: usize, m: usize, a: [u32; m]);

    let mut s = vec![0; n];
    for a in a {
        let pos = s.partition_point(|&s| s >= a);
        if pos == n {
            putln!("-1");
        } else {
            putln!(pos + 1);
            unsafe { *s.get_unchecked_mut(pos) = a }
        }
    }
}

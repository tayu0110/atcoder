use iolib::{putln, scan};

fn main() {
    scan!(_n: usize, q: usize, s: String);

    let mut t = vec![0i32; s.len()];
    for (i, s) in s.as_bytes().windows(2).enumerate() {
        unsafe {
            *t.get_unchecked_mut(i + 1) =
                *t.get_unchecked(i) + (s.get_unchecked(0) == s.get_unchecked(1)) as i32;
        }
    }

    for _ in 0..q {
        scan!(l: usize, r: usize);
        putln!(unsafe { t.get_unchecked(r - 1) - t.get_unchecked(l - 1) });
    }
}

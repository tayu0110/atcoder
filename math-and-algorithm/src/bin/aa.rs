use iolib::{putvec_with_spaceln, scan};

fn main() {
    scan!(n: usize, a: [u32; n]);
    let mut a = a;
    a.sort_unstable();
    putvec_with_spaceln!(a);
}

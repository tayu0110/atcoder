use proconio::*;

fn main() {
    input! {n: usize, mut a: [u32; n]}

    let mut res = 0;
    let mut a = &mut a[..];
    while a.len() >= 2 {
        let &max = a.iter().max().unwrap();
        if max == 0 {
            break;
        }
        let msb = 1 << (31 - max.leading_zeros());
        let (mut l, mut r) = (0, 0);
        while r < a.len() {
            if a[r] >= msb {
                a.swap(l, r);
                l += 1;
            }
            r += 1;
        }
        a[..l].iter_mut().for_each(|a| *a ^= msb);
        if l >= 2 {
            a = &mut a[..l];
            res += msb << 1;
        }
    }
    println!("{res}")
}

use proconio::*;

fn main() {
    input! {b: [u8; 10], n: usize, mut a: [marker::Bytes; n]}
    a.iter_mut().for_each(|v| {
        v.reverse();
        v.resize(20, b'0');
        v.reverse();
        v.iter_mut().for_each(|v| {
            *v -= b'0';
            for i in 0..10 {
                if b[i] == *v {
                    *v = i as u8;
                    break;
                }
            }
        });
    });

    a.sort();
    a.iter_mut().for_each(|v| {
        v.iter_mut().for_each(|v| {
            let idx = *v as usize;
            *v = b[idx];
        });
        while v[0] == 0 {
            v.remove(0);
        }
        println!("{}", v.iter().fold(0usize, |s, v| s * 10 + *v as usize));
    });
}

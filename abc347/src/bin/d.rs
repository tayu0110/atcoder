use proconio::*;

fn main() {
    input! {a: usize, b: usize, c: u64}

    let pc = c.count_ones() as usize;

    if a + b < pc {
        println!("-1");
        return;
    }

    let k = a + b - pc;
    if k % 2 != 0 {
        println!("-1");
        return;
    }

    let mut z = k / 2;

    if a < z || b < z {
        println!("-1");
        return;
    }

    let mut oa = a - z;
    let mut ob = b - z;

    let mut ra = 0u64;
    let mut rb = 0u64;
    for i in 0..60 {
        if c & (1 << i) == 0 {
            if z > 0 {
                z -= 1;
                ra |= 1 << i;
                rb |= 1 << i;
            }
        } else {
            if oa > 0 {
                oa -= 1;
                ra |= 1 << i;
            } else if ob > 0 {
                ob -= 1;
                rb |= 1 << i;
            } else {
                println!("-1");
                return;
            }
        }
    }

    if z > 0 || oa > 0 || ob > 0 {
        println!("-1");
        return;
    }

    assert_eq!(ra.count_ones() as usize, a);
    assert_eq!(rb.count_ones() as usize, b);
    assert_eq!(ra ^ rb, c);

    println!("{ra} {rb}");
}

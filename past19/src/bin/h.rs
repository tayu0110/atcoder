use std::fmt::Write;

use permutohedron::LexicalPermutation;
use proconio::*;

fn main() {
    input! {n: usize, s: usize, mut a: [usize; n]}

    let mut stack = vec![];
    a.sort_unstable();
    while {
        for mut i in 0..1 << (n - 1) {
            let mut keep = i;
            stack.push(a[0]);
            for &a in &a[1..] {
                if i & 1 != 0 {
                    stack.push(a);
                } else {
                    let l = stack.last_mut().unwrap();
                    *l *= a;
                }
                i >>= 1;
            }
            let sum = stack.drain(..).fold(0usize, |s, v| s + v);
            if sum == s {
                let mut buf = String::with_capacity(30);
                writeln!(buf, "Yes").ok();
                write!(buf, "{}", a[0]).ok();
                for &a in &a[1..] {
                    if keep & 1 != 0 {
                        write!(buf, "+").ok();
                    } else {
                        write!(buf, "x").ok();
                    }
                    write!(buf, "{a}").ok();
                    keep >>= 1;
                }
                println!("{buf}");
                return;
            }
        }
        a.next_permutation()
    } {}
    println!("No")
}

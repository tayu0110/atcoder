#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {h: usize, m: usize}
    let ph = h;

    for h in h..h+24 {
        let m = if h == ph {
            m
        } else {
            0
        };
        for m in m..60 {
            let h = h % 24;
            let a = h / 10;
            let b = h % 10;
            let c = m / 10;
            let d = m % 10;

            let (nh, nm) = (a * 10 + c, b * 10 + d);

            if nh < 24 && nm < 60 {
                println!("{} {}", h, m);
                return;
            }
        }
    }
}

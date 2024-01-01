use ds::FixedRingQueue;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [u16; n]}

    let mut toggle = vec![];
    for _ in 0..m {
        input! {x: u8, y: u8, z: u8}
        toggle.push(((1 << x) | (1 << y) | (1 << z)) >> 1);
    }

    let mut dist = [u16::MAX; 1024];
    let start = a.into_iter().rev().fold(0, |s, v| (s << 1) | v);
    dist[start as usize] = 0;
    let mut nt: FixedRingQueue<u16, 1024> = FixedRingQueue::new();
    nt.push(start);
    while let Some(now) = nt.pop() {
        for &t in &toggle {
            if dist[(now ^ t) as usize] == u16::MAX {
                dist[(now ^ t) as usize] = dist[now as usize] + 1;
                nt.push(now ^ t);
            }
        }
    }

    println!("{}", dist[(1 << n) - 1] as i16)
}

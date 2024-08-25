use proconio::*;
use rustc_hash::FxHashMap;

fn main() {
    input! {h: usize, w: usize, n: usize, mut a: [usize; n]}

    a.sort_unstable();

    let mut rem = FxHashMap::default();
    rem.insert((h, w), 1);
    while let Some(a) = a.pop() {
        let kv = rem.into_iter().collect::<Vec<_>>();
        let mut next = FxHashMap::default();
        let na = 1 << a;
        for ((h, w), rem) in kv {
            if h >= na {
                let dh = h / na;
                let dw = w / na;
                *next.entry((na, na)).or_insert(0) += dh * dw * rem;
                if h % na != 0 {
                    let h = h % na;
                    let w = na * dw;
                    *next.entry((h.min(w), h.max(w))).or_insert(0) += rem;
                }
                if w % na != 0 {
                    let h = na * dh;
                    let w = w % na;
                    *next.entry((h.min(w), h.max(w))).or_insert(0) += rem;
                }
                if h % na != 0 && w % na != 0 {
                    let h = h % na;
                    let w = w % na;
                    *next.entry((h.min(w), h.max(w))).or_insert(0) += rem;
                }
            } else {
                *next.entry((h, w)).or_insert(0) += rem;
            }
        }

        rem = next;
        if rem.contains_key(&(1 << a, 1 << a)) {
            *rem.get_mut(&(1 << a, 1 << a)).unwrap() -= 1;
            if *rem.get(&(1 << a, 1 << a)).unwrap() == 0 {
                rem.remove(&(1 << a, 1 << a));
            }
        } else {
            println!("No");
            return;
        }
    }

    println!("Yes")
}

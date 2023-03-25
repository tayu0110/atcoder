use proconio::{input, marker::Chars};
use segtree::SegmentTree;

fn main() {
    input! {n: usize, mut s: Chars, q: usize}

    let mut st = SegmentTree::new(n, 0u32, |&l, &r| l | r);

    for (i, &c) in s.iter().enumerate() {
        st.set(i, 1 << (c as u8 - b'a'));
    }

    for _ in 0..q {
        input! {t: usize}

        if t == 1 {
            input! {i: usize, c: char}

            if s[i - 1] == c {
                continue;
            }

            let old_c = s[i - 1];
            let val = (1 << (old_c as u8 - b'a')) | (1 << (c as u8 - b'a'));
            st.update_by(i - 1, val, |&old, &new| old ^ new);

            s[i - 1] = c;
        } else {
            input! {l: usize, r: usize}

            let k = st.foldl(l - 1, r);
            println!("{}", k.count_ones());
        }
    }
}

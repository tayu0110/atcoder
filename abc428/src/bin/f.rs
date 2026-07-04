use std::fmt::Write as _;

use ds::{LazySegmentTree, MapMonoid};
use proconio::*;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

struct T;
impl MapMonoid for T {
    // direction, position, turn
    type M = (Direction, usize, usize);
    type Act = (Direction, usize, usize);

    fn e() -> Self::M {
        (Direction::Left, 0, 0)
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        if l.2 > r.2 {
            *l
        } else {
            *r
        }
    }
    fn id() -> Self::Act {
        Self::e()
    }
    fn composite(l: &Self::Act, r: &Self::Act) -> Self::Act {
        Self::op(l, r)
    }
    fn map(m: &Self::M, act: &Self::Act) -> Self::M {
        Self::op(m, act)
    }
}

fn main() {
    input! {n: usize, w: [usize; n], q: usize}

    let mut buf = String::new();
    let mut st = LazySegmentTree::<T>::new(n);
    for i in 1..q + 1 {
        input! {ty: u8}

        if ty == 1 {
            input! {v: usize}
            let (direction, position, _) = st.get(v - 1);
            let left = match direction {
                Direction::Left => position,
                Direction::Right => position.wrapping_sub(w[v - 1]),
            };

            st.apply(..v, (Direction::Left, left, i));
        } else if ty == 2 {
            input! {v: usize}
            let (direction, position, _) = st.get(v - 1);
            let right = match direction {
                Direction::Left => w[v - 1].wrapping_add(position),
                Direction::Right => position,
            };

            st.apply(..v, (Direction::Right, right, i));
        } else {
            input! {x: usize}

            let (mut l, mut r) = (-1, n as i32);
            while r - l > 1 {
                let m = (r + l) / 2;

                let (direction, position, _) = st.get(m as usize);
                let left = match direction {
                    Direction::Left => position,
                    Direction::Right => position.wrapping_sub(w[m as usize]),
                };

                if (left..left + w[m as usize]).contains(&x) {
                    r = m;
                } else {
                    l = m;
                }
            }

            writeln!(buf, "{}", n - r as usize).unwrap();
        }
    }

    print!("{buf}")
}

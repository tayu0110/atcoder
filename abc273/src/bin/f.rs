#[allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

// #[fastout]
fn main() {
    input! {n: usize, x: i64, y: [i64; n], z: [i64; n]}

    // Type : Takahashi=0, goal=1, wall=2, hammer=3
    // (coordinate, type, index)
    let mut items = vec![(0, 0, 0), (x, 1, 0)];
    for (i, v) in y.iter().enumerate() {
        items.push((*v, 2, i));
    }
    for (i, v) in z.iter().enumerate() {
        items.push((*v, 3, i));
    }

    items.sort();

    let mut tak = 0;
    for (i, &(_co, tp, _)) in items.iter().enumerate() {
        if tp == 0 {
            tak = i;
            break;
        }
    }

}

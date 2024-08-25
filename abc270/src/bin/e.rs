#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, mut k: usize, mut a: [usize; n]}

    let mut nt = std::collections::BinaryHeap::new();
    for (i, v) in a.iter().enumerate() {
        if *v != 0 {
            nt.push(std::cmp::Reverse((*v, i)));
        }
    }

    let mut g = 0usize;
    while let Some(std::cmp::Reverse((rem, idx))) = nt.pop() {
        let size = nt.len() + 1;

        if (rem - g) * size <= k {
            a[idx] = 0;
            k -= (rem - g) * size;
            g = rem;
        } else {
            nt.push(std::cmp::Reverse((rem, idx)));
            break;
        }
    }

    let mut indices = nt.into_iter().map(|std::cmp::Reverse((_, idx))| idx).collect::<Vec<_>>();
    indices.sort();
    let size = indices.len();

    if size > 0 {
        let mut rem = k % size;
        for i in indices {
            a[i] -= g;
            a[i] -= k / size;
            if rem > 0 {
                a[i] -= 1;
                rem -= 1;
            }
        }
    }

    for i in 0..n {
        if i > 0 {
            print!(" ");
        }
        print!("{}", a[i]);
    }
    println!();
}

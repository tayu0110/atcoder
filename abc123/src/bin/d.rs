#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {x: usize, y: usize, z: usize, k: usize, mut a: [usize; x], mut b: [usize; y], mut c: [usize; z]}
    let mut t = vec![];
    for a in a.iter().take(x) {
        for b in b.iter().take(y) {
            t.push(*a + *b);
        }
    }
    t.sort();
    t.reverse();
    c.sort();
    c.reverse();

    let mut nt = std::collections::BinaryHeap::new();
    for t in t.iter().take(x + y) {
        for c in c.iter().take(z) {
            let s = t + c;
            if nt.len() < k {
                nt.push(std::cmp::Reverse(s));
            } else {
                let std::cmp::Reverse(f) = nt.pop().unwrap();
                if s > f {
                    nt.push(std::cmp::Reverse(s));
                } else {
                    nt.push(std::cmp::Reverse(f));
                    break;
                }
            }
        }
    }

    let mut res = vec![];
    while let Some(std::cmp::Reverse(v)) = nt.pop() {
        res.push(v);
    }
    res.reverse();

    for v in res {
        println!("{}", v);
    }
}

// #[allow(unused_imports)]
// use proconio::{input, marker::Chars, source::line::LineSource};

// fn main() {
//     input! {
//         x: usize,
//         y: usize,
//         z: usize,
//         k: usize,
//         mut a: [usize; x],
//         mut b: [usize; y],
//         mut c: [usize; z]
//     }

//     let mut t = vec![];
//     for v in a {
//         for w in &b {
//             t.push(v + *w);
//         }
//     }
//     t.sort();
//     t.reverse();
//     c.sort();
//     c.reverse();

//     let mut nt = std::collections::BinaryHeap::new();
//     for v in t {
//         for w in &c {
//             let s = v + *w;
//             if nt.len() < k {
//                 nt.push(std::cmp::Reverse(s));
//             } else {
//                 let std::cmp::Reverse(f) = nt.pop().unwrap();
//                 if s > f {
//                     nt.push(std::cmp::Reverse(s));
//                 } else {
//                     nt.push(std::cmp::Reverse(f));
//                     break;
//                 }
//             }
//         }
//     }

//     let mut res = vec![];
//     while let Some(std::cmp::Reverse(v)) = nt.pop() {
//         res.push(v);
//     }
//     res.reverse();

//     for v in res {
//         println!("{}", v);
//     }
// }

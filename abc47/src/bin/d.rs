use proconio::*;

fn main() {
    input! {n: usize, _t: usize, a: [usize; n]}

    let mut max_benefit = 0;
    let mut max_a = 0;
    for &a in a.iter().rev() {
        if a < max_a {
            max_benefit = std::cmp::max(max_benefit, max_a - a);
        }

        max_a = std::cmp::max(max_a, a);
    }

    let mut res = 0;
    let mut nt = std::collections::BinaryHeap::new();
    for &a in a.iter().rev() {
        while let Some(na) = nt.pop() {
            if na != a + max_benefit {
                nt.push(na);
                break;
            }

            res += 1;
        }

        nt.push(a);
    }

    println!("{}", res);
}

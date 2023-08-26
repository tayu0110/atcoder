use fenwick_tree::FenwickTree;
use proconio::input;

fn main() {
    input! {n: usize, mut a: [usize; n], mut b: [usize; n]}

    {
        let mut a = a.clone();
        let mut b = b.clone();
        a.sort();
        b.sort();
        if a != b {
            println!("No");
            return;
        }
        a.dedup();
        b.dedup();

        if a.len() != n {
            println!("Yes");
            return;
        }
    }

    let f = |a: Vec<usize>| {
        let mut res = 0;
        let mut ft = FenwickTree::new(n + 1, 0);
        for a in a {
            res += ft.get_sum(a, n + 1);
            ft.add(a, 1);
        }
        res
    };

    let (ra, rb) = (f(a), f(b));

    if (ra + rb) % 2 == 0 {
        println!("Yes")
    } else {
        println!("No")
    }
}

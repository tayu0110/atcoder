use proconio::*;
use segtree::range_add_range_minimum_query;

fn main() {
    input! {n: usize, c: [i64; n], q: usize}

    let init = c.iter().sum::<i64>();

    let convert = |idx: usize| -> usize {
        if idx % 2 == 0 {
            idx / 2
        } else {
            (n + 1) / 2 + idx / 2
        }
    };
    let mut st = range_add_range_minimum_query(n);
    for (i, c) in c.into_iter().enumerate() {
        let i = convert(i);
        st.set(i, c);
    }

    for _ in 0..q {
        input! {t: usize}

        if t == 1 {
            input! {x: usize, a: i64}
            let x = convert(x - 1);

            let min = st.prod(x, x + 1);
            if min < a {
                continue;
            }
            st.apply(x, -a)
        } else if t == 2 {
            input! {a: i64}

            let min = st.prod(0, (n + 1) / 2);
            if min < a {
                continue;
            }

            st.apply_range(0, (n + 1) / 2, -a);
        } else {
            input! {a: i64}

            let min = st.all_prod();
            if min < a {
                continue;
            }

            st.apply_range(0, n, -a);
        }
    }

    println!("{}", init - (0..n).map(|i| st.prod(i, i + 1)).sum::<i64>())
}

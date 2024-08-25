use proconio::*;
use segtree::{LazySegtree, MapMonoid};

struct F64Sum;

impl MapMonoid for F64Sum {
    type Act = f64;
    type E = (f64, f64);
    fn e() -> Self::E {
        (0.0, 1.0)
    }
    fn id() -> Self::Act {
        0.0
    }
    fn op(l: Self::E, r: Self::E) -> Self::E {
        (l.0 + r.0, l.1 + r.1)
    }
    fn composite(l: Self::Act, r: Self::Act) -> Self::Act {
        l + r
    }
    fn map(act: Self::Act, elem: Self::E) -> Self::E {
        (elem.0 + act * elem.1, elem.1)
    }
}

fn main() {
    input! {n: usize, l: usize, d: usize}

    let mut dpx = LazySegtree::<F64Sum>::new(n + d + 1);
    let mut dpy = LazySegtree::<F64Sum>::new(n + d + 1);
    dpx.set(0, (1.0, 1.0));
    dpy.set(0, (1.0, 1.0));

    let mut res = 0.0;
    let mut sum = 0.0;
    for i in 0..n {
        let (p, _) = dpx.get(i);
        dpx.set(i, (0.0, 1.0));
        dpx.apply_range(i + 1, i + 1 + d, p / d as f64);
        {
            let (p, _) = dpy.get(i);
            dpy.set(i, (0.0, 1.0));
            dpy.apply_range(i + 1, i + 1 + d, p / d as f64);
            sum += dpy.prod(0, i.min(l + 1)).0;
        }
        res = (p * sum).max(res);
    }

    println!("{res}");
}

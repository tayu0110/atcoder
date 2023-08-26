use proconio::input;
use unionfind::UnionFind;

fn main() {
    input! {n: usize, mut p: [usize; n-1], q: usize}
    p.iter_mut().for_each(|v| *v -= 1);
    p.insert(0, 0);

    let mut uf = UnionFind::new(n);
    let mut min = (0..n).collect::<Vec<_>>();
    let get_min = |i: usize, uf: &mut UnionFind, min: &Vec<usize>| min[uf.root(i)];

    for _ in 0..q {
        input! {t: usize}

        if t == 1 {
            input! {mut u: usize, mut v: usize}
            u -= 1;
            v -= 1;

            while get_min(u, &mut uf, &mut min) > v {
                let ru = get_min(u, &mut uf, &mut min);
                let rru = get_min(p[ru], &mut uf, &mut min);
                uf.merge(ru, p[ru]);
                min[uf.root(ru)] = rru;
            }
        } else {
            input! {x: usize}
            println!("{}", get_min(x - 1, &mut uf, &mut min) + 1);
        }
    }
}

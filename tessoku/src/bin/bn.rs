use proconio::*;
use unionfind::UnionFind;

#[fastout]
fn main() {
    input! {n: usize, q: u32}

    let mut uf = UnionFind::new(n);
    for _ in 0..q {
        input! {ty: u8}

        if ty == 1 {
            input! {u: u32, v: u32}
            uf.merge(u as usize - 1, v as usize - 1);
        } else {
            input! {u: u32, v: u32}
            println!(
                "{}",
                if uf.is_same(u as usize - 1, v as usize - 1) { "Yes" } else { "No" }
            )
        }
    }
}

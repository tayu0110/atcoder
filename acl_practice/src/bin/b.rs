use proconio::input;

#[proconio::fastout]
fn main() {
    input! {n: usize, q: usize, a: [usize; n], p: [(u8, u32, usize); q]}

    let mut ft = FenwickTree::new(n, 0);
    for (i, v) in a.into_iter().enumerate() {
        ft.add(i, v);
    }

    for (t, p, x) in p {
        if t == 0 {
            ft.add(p as usize, x);
        } else {
            println!("{}", ft.get_sum(p as usize, x));
        }
    }
}

struct FenwickTree<T> {
    size: usize,
    def_val: T,
    tree: Vec<T>
}
#[allow(dead_code)]
impl<T> FenwickTree<T>
where T: Sized + Clone + Copy + Default + std::ops::AddAssign + std::ops::Sub<Output = T> + std::ops::SubAssign + PartialOrd
{
    fn new(size: usize, def_val: T) -> Self {
        Self { size: size+1, def_val, tree: vec![def_val; size+1] }
    }
    fn add(&mut self, idx: usize, val: T) {
        let mut idx = idx as i64;
        idx += 1;
        while (idx as usize) < self.size {
            self.tree[idx as usize] += val;
            idx += idx & -idx;
        }
    }
    fn get_sum_sub(&self, r: usize) -> T {
        let mut r = r as i64;
        if (r as usize) >= self.size {
            r = self.size as i64 - 1;
        }
        let mut res = self.def_val;
        while r > 0 {
            res += self.tree[r as usize];
            r -= r & -r;
        }
        res
    }
    fn get_sum(&self, l: usize, r: usize) -> T {
        self.get_sum_sub(r) - self.get_sum_sub(l)
    }
    fn lower_bound(&self, mut val: T) -> usize {
        let mut now = 0;
        let mut n = 1;
        while n * 2 <= self.size {
            n *= 2;
        }
        while n > 0 {
            if now + n < self.size && self.tree[now + n] < val {
                val -= self.tree[now + n];
                now += n;
            }
            n /= 2;
        }
        now
    }
    fn upper_bouond(&self, mut val: T) -> usize {
        let mut now = 0;
        let mut n = 1;
        while n * 2 < self.size {
            n *= 2;
        }
        while n > 0 {
            if now + n < self.size && self.tree[now + n] <= val {
                val -= self.tree[now + n];
                now += n;
            }
            n /= 2;
        }
        now
    }
}
use proconio::*;

const F: [f32; 100001] = {
    let mut i = 1;
    let mut f = [0.0f32; 100001];
    while i < 100001 {
        let mut x = f[i - 1];
        let mut j = 0;
        while j < 10 {
            x = (2.0 * x * x * x + i as f32) / (3.0 * x * x + 1.0);
            j += 1;
        }
        f[i] = x;
        i += 1;
    }
    f
};

fn main() {
    input! {n: usize}
    println!("{:.4}", F[n])
}

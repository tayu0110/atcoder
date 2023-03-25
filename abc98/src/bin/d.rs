use proconio::input;

fn main() {
    input! {n: usize, a: [u32; n]}

    let (mut l, mut r) = (0, 0);
    let (mut sum, mut xor) = (0, 0);
    let mut res = 0;
    while l < n {
        while r < n && sum + a[r] == xor ^ a[r] {
            sum += a[r];
            xor ^= a[r];
            r += 1;
        }

        res += r - l;

        sum -= a[l];
        xor ^= a[l];
        l += 1;
    }

    println!("{}", res)
}

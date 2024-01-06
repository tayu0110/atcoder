use proconio::input;

// fn totient(mut x: usize) -> usize {
//     let mut buf = vec![];
//     let max = x;
//     for i in (2..=max).take_while(|i| *i * *i <= max) {
//         let mut j = 1;
//         let mut k = 0;
//         while x % i == 0 {
//             j *= i;
//             k += 1;
//             x /= i;
//         }

//         if k != 0 {
//             buf.push((i, j));
//         }
//     }
//     if x != 1 {
//         buf.push((x, x));
//     }

//     let mut res = 1;
//     for (i, j) in buf {
//         res *= i / j * (j - 1);
//     }

//     res
// }

fn main() {
    input! {_a: usize, _m: usize, _k: usize}
}

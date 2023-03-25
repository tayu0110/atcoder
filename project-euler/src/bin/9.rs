fn main() {
    for i in 1..=1000 {
        for j in (i..=1000).take_while(|j| i + *j <= 1000) {
            for k in (j..=1000).take_while(|k| i + j + *k <= 1000) {
                if i + j + k == 1000 && i*i + j*j == k*k {
                    println!("{}", i*j*k);
                }
            }
        }
    }
}
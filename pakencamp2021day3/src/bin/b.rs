use proconio::input;
	
fn main() {
    input! {n: usize, x: usize, y: usize, a: [usize; n]};

    let sum = a.iter().fold(0usize, |sum, x| sum + *x);

    if sum != x + y {
        println!("No");
        std::process::exit(0);
    }

    let mut b = a.clone();
    let mut a = a;
    a.append(&mut b);
    
    let mut l = 0;
    let mut r = 0;

    let mut sum = 0;
    while l < a.len() {
        r = std::cmp::max(l, r);
        while r < a.len() && sum < x {
            sum += a[r];
            r += 1;
        }

        if sum == x {
            println!("Yes");
            std::process::exit(0);
        }

        sum -= a[l];
        l += 1;
    }

    println!("No");
}
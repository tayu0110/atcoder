fn main() {
    const N: usize = 100;
    println!("{}", N*N*(N+1)*(N+1)/4-(1..=N).map(|v| v*v).sum::<usize>());
}
fn fib(n: u64) -> u64 {
    if n <= 1 {
        return n
    }
    fib(n - 1) + fib(n - 2)
}

fn main() {
    println!("Fibonacci Numbers:");
    for k in 0..11 {
        println!("  {}", fib(k));
    }
}

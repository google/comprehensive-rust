fn fib(n: u32) -> u32 { if n < 2 { n } else { fib(n - 1) + fib(n - 2) } } fn main() { println!("fib(20) = {}", fib(20)); }

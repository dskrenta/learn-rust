fn main() {
    println!("{}", fib(10));
    println!("{}", fib(7));
    println!("{}", fib(15));
}

fn fib(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }

    fib(n - 1) + fib(n - 2)
}

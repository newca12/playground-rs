pub fn fibonacci(n: u64) -> u64 {
    fn fib(n: u64) -> (u64, u64) {
        if n == 0 {
            return (0, 1);
        }
        let (a, b) = fib(n / 2);
        let c = a.wrapping_mul(b.wrapping_mul(2).wrapping_sub(a));
        let d = a.wrapping_mul(a).wrapping_add(b.wrapping_mul(b));
        if n % 2 == 0 {
            (c, d)
        } else {
            (d, c.wrapping_add(d))
        }
    }
    fib(n).0
}

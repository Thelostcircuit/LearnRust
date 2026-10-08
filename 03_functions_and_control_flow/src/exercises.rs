// Lesson 3 exercises. Replace each `todo!()` and run `cargo test`.
#![allow(dead_code, unused_variables)]

/// Return the larger of two numbers. Use `if` as an expression.
pub fn max_of(a: i32, b: i32) -> i32 {
    todo!()
}

/// Return "Fizz" if n is divisible by 3, "Buzz" if by 5, "FizzBuzz" if by both,
/// otherwise the number itself as a string (use `n.to_string()` or `format!`).
pub fn fizzbuzz(n: u32) -> String {
    todo!()
}

/// Sum every number from 1 to n inclusive, using a `for` loop.  sum_to(4) == 1+2+3+4 == 10
pub fn sum_to(n: u32) -> u32 {
    todo!()
}

/// n factorial: 5! = 5*4*3*2*1 = 120, and 0! = 1. Use a `while` or `for` loop.
pub fn factorial(n: u64) -> u64 {
    todo!()
}

/// Return true if n is prime (only divisible by 1 and itself). 0 and 1 are not prime.
pub fn is_prime(n: u32) -> bool {
    todo!()
}

/// The Collatz sequence: if n is even, divide by 2; if odd, do 3n + 1. Repeat until n == 1.
/// Return how many steps it takes.  collatz_steps(1) == 0, collatz_steps(6) == 8
/// Try using `loop` with `break`.
pub fn collatz_steps(n: u64) -> u32 {
    todo!()
}

/// Return the nth Fibonacci number: fib(0)=0, fib(1)=1, fib(n)=fib(n-1)+fib(n-2)
/// Bonus: do it with a loop instead of recursion.
pub fn fib(n: u32) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_of() {
        assert_eq!(max_of(1, 2), 2);
        assert_eq!(max_of(5, -5), 5);
        assert_eq!(max_of(3, 3), 3);
    }

    #[test]
    fn test_fizzbuzz() {
        assert_eq!(fizzbuzz(1), "1");
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(10), "Buzz");
        assert_eq!(fizzbuzz(30), "FizzBuzz");
    }

    #[test]
    fn test_sum_to() {
        assert_eq!(sum_to(0), 0);
        assert_eq!(sum_to(4), 10);
        assert_eq!(sum_to(100), 5050);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3_628_800);
    }

    #[test]
    fn test_is_prime() {
        let primes: Vec<u32> = (0..30).filter(|&n| is_prime(n)).collect();
        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn test_collatz_steps() {
        assert_eq!(collatz_steps(1), 0);
        assert_eq!(collatz_steps(6), 8);
        assert_eq!(collatz_steps(27), 111);
    }

    #[test]
    fn test_fib() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(10), 55);
        assert_eq!(fib(50), 12_586_269_025);
    }
}

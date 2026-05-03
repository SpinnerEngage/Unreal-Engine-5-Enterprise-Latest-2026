fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() {
    let num = 10;
    let result = fibonacci(num);
    println!("Fibonacci of {} is {}", num, result);
}

fn factorial(n: u32) -> u32 {
    if n == 0 {
        return 1;
    }
    n * factorial(n - 1)
}

fn main() {
    let num = 5;
    let result = factorial(num);
    println!("Factorial of {} is {}", num, result);
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let a = 48;
    let b = 18;
    let result = gcd(a, b);
    println!("GCD of {} and {} is {}", a, b, result);
}

fn main() {
    let num = 16;
    let sqrt_result = (num as f64).sqrt();
    println!("Square root of {} is {}", num, sqrt_result);
}

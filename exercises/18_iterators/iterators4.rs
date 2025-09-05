fn factorial(num: u64) -> u64 {
    // TODO: Complete this function to return the factorial of `num` which is
    // defined as `1 * 2 * 3 * … * num`.
    // https://en.wikipedia.org/wiki/Factorial
    //
    // Do not use:
    // - early returns (using the `return` keyword explicitly)
    // Try not to use:
    // - imperative style loops (for/while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // if num == 0 { 1 } else { (1..=num).product() }
    // match num {
    //     0 => 1,
    //     n => (1..=n).product()
    // }
    // (1..=num).fold(1, |acc, x| acc * x)

    (1..=num).product::<u64>()
}

fn main() {
    // You can optionally experiment here.
    let num = 0;
    let range = 1..=num; 
    println!("{:?}", range.product::<i32>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial(4), 24);
    }
}

// Rewrite the factorial function using a `while` loop.
pub fn factorial(n: u32) -> u32 {
    let mut fact: u32 = 1;
    let mut current_value = n;
    if n == 0 {
        1
    } else {
        while current_value > 1 {
            fact *= current_value;
            current_value -= 1;
        }

        fact
    }
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}

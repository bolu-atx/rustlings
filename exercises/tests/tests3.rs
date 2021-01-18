// tests3.rs
// This test isn't testing our function -- make it do that in such a way that
// the test passes. Then write a second test that tests whether we get the result
// we expect to get when we call `is_even(5)`.
// Execute `rustlings hint tests3` for hints :)


pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(20));
        assert!(is_even(22));
        assert!(is_even(24));
        assert!(is_even(26));
        assert!(is_even(28));
    }

    #[test]
    fn is_false_when_odd() {
        let mut nums = [1,3,5,7,9];
        for num in nums.iter_mut()
        {
            assert!(!is_even(*num));
            *num = 3;
        }

    }

    #[test]
    fn test_even_5()
    {
        assert!(!is_even(5));
    }
}

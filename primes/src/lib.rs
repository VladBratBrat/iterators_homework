pub fn is_prime(n: u64) -> bool {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::is_prime;

    #[test]
    fn small() {
        assert_eq!(is_prime(0), false);
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(10), false);
        assert_eq!(is_prime(20), false);
    }

    #[test]
    #[ntest::timeout(5000)]
    fn test_big() {
        let a = [1_000_003, 1_000_159, 1_000_253, 1_000_393];
        for x in a.iter() {
            for y in a.iter() {
                assert_eq!(is_prime(x * y), false)
            }
        }

        assert_eq!(is_prime(100_000_000_003), true);
        assert_eq!(is_prime(200_000_000_041), true);
    }
}

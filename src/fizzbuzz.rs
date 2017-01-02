pub fn fizzbuzz(number: i32) -> String {
    let is_divisible_by_3: bool =  number % 3 == 0;
    let is_divisible_by_5: bool =  number % 5 == 0;

    if is_divisible_by_3 && is_divisible_by_5 {
        "fizzbuzz".to_string()
    } else if is_divisible_by_3 {
        "fizz".to_string()
    } else if is_divisible_by_5  {
        "buzz".to_string()
    } else {
        number.to_string()
    }
}

#[cfg(test)]
mod fizzbuzz_test {
    use super::fizzbuzz;

    #[test]
    fn when_a_number_is_given_returns_the_number() {
        assert_eq!(fizzbuzz(1), "1");
        assert_eq!(fizzbuzz(4), "4");
    }

    #[test]
    fn when_a_multiple_of_3_is_given_returns_fizz() {
        assert_eq!(fizzbuzz(3), "fizz");
        assert_eq!(fizzbuzz(9), "fizz");
    }

    #[test]
    fn when_a_multiple_of_5_is_given_returns_fizz() {
        assert_eq!(fizzbuzz(10), "buzz");
        assert_eq!(fizzbuzz(25), "buzz");
    }

    #[test]
    fn when_a_multiple_of_3_and_5_is_given_returns_fizzbuzz() {
        assert_eq!(fizzbuzz(15), "fizzbuzz");
        assert_eq!(fizzbuzz(30), "fizzbuzz");
    }
}

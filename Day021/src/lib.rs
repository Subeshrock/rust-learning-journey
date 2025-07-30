// by default all tests are run in parallel
// to run tests sequentially, use `cargo test -- --test-threads=1`
// to run a specific test, use `cargo test it_works`
// to run tests with a specific name, use `cargo test it_works2`
// print statements will not be shown by default
// to show print statements, use `cargo test -- --show-output`
// we can run test by using some part of test name like to run it_works and it_owrks2 we can use cargo test it_
// we can run tests by specifying module name like `cargo test tests::`


fn prints_and_returns_10(a: i32) -> i32 {
    println!("This will be printed {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("Two plus two does not equal four"))
        }
    }

    #[test]
    fn it_works2() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(5);
        assert_eq!(value, 10);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(value, 5, "This test will fail because the value is not 5");
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // This test is ignored by default
        // It can be run with `cargo test -- --ignored`
        assert_eq!(2 + 2, 4);
        println!("This is an expensive test that is ignored by default");
    }
}

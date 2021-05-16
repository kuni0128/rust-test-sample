mod rectangle;
mod greeting;
mod guess;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = rectangle::Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = rectangle::Rectangle {
            width: 7,
            height: 6,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_not_hold_larger() {
        let larger = rectangle::Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = rectangle::Rectangle {
            width: 7,
            height: 6,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn greeting_containes_name() {
        let result = greeting::greeting("Jack");
        assert!(
            result.contains("Jack"),
            "Greeting did not contain name, value was `{}`", result
        );
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        guess::Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100_expected() {
        guess::Guess::new(200);
    }

    #[test]
    fn return_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

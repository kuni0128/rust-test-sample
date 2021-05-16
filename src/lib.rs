mod rectangle;

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
}

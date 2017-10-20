#[cfg(test)]
mod tests {
    // #[test]
    // fn it_works() {
    // }
    use super::*;
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
    // #[test]
    // fn large_can_hold_smaller() {
    //     let large = Rectangle {length: 100, width: 100};
    //     let small = Rectangle {length: 20, width: 20};
    //     assert!(large.can_hold(&small));

    // }
    // #[test]
    // fn exploration() {
        
    // }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }
}

#[derive(Debug)]
pub struct Rectangle {	
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

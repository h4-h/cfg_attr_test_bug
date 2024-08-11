fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
struct MyStruct {
    pub id: i32,
}

#[cfg(test)]
mod test {
    use subcrate::MyStruct; 
    /* Comment line above and uncomment line bellow. */
    // use super::MyStruct;

    #[test]
    fn check_equality() {
        let first = MyStruct { id: 1 };
        let second = MyStruct { id: 2 };

        assert_eq!(first, second);
    }
}

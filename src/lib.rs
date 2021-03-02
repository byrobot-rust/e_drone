// An attribute to hide warnings for unused code.
#![allow(dead_code)]


mod base;
mod communication;
mod protocol;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

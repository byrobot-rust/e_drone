// An attribute to hide warnings for unused code.
#![allow(dead_code)]


pub mod base;
pub mod communication;
pub mod protocol;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

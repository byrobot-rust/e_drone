// An attribute to hide warnings for unused code.
#![allow(dead_code)]


pub mod communication;
pub mod file;
pub mod protocol;
pub mod system;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod bolt;
pub mod errors;

#[macro_use] 
extern crate log;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

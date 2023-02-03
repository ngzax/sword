#[macro_use]
extern crate num_derive;
pub mod interpreter;
pub mod jets;
pub mod mem;
pub mod mug;
pub mod noun;
pub mod serialization;

#[cfg(test)]
mod tests {

    #[test]
    fn tas() {
        use ares_macros::tas;
        assert_eq!(tas!(b"cut"), 0x747563);
        assert_eq!(tas!(b"dec"), 0x636564);
    }
}

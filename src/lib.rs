//! TODO: Add a description of the library here.
//! This an agent core libary, which provides the basic functionalities for the agent.

#![deny(missing_docs)]

/// A simple library to test the build process of a Rust project.
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

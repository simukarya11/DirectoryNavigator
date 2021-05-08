#![allow(non_snake_case)]

pub mod DirectorySearchEventTrait;
mod directory_navigator;

pub use DirectorySearchEventTrait::*;
pub use directory_navigator::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

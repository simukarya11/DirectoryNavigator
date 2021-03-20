#![allow(non_snake_case)]
use std::path::{Path};

#[allow(non_snake_case)]
pub trait DirectorySearchEvent {
    fn setSearchDirectory(&mut self, directory: &Path);
    fn searchFile(&mut self, foundFilePath: &Path);
}

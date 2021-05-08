#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::DirectorySearchEventTrait::directory_search_event_trait::DirectorySearchEventTrait;
use std::path::{Path, PathBuf};
use std::{io, fs};
use std::io::{Error, ErrorKind};
use std::fs::DirEntry;

type SearchPatterns = Vec<PathBuf>;

pub struct DirectoryNavigator {
    patterns: SearchPatterns,
    /// instance of App : DirEvent, requires impl of DirectorySearchEventTrait
    textSearch: Option<Box<dyn DirectorySearchEventTrait>>,
    /// number of files processed
    num_files: usize,
    /// number of dirs processed
    num_directories: usize,
    /// recurse ?
    recurse: bool,
}

impl DirectoryNavigator {
    pub fn new() -> Self {
        Self {
            patterns: SearchPatterns::new(),
            textSearch: None,
            num_files: 0,
            num_directories: 0,
            recurse: true,
        }
    }

    /// visits are recursive?
    pub fn recurse(&mut self, p: bool) {
        self.recurse = p;
    }

    /// return reference to App to configure, get results
    pub fn get_app(&mut self) -> &mut Option<Box<dyn DirectorySearchEventTrait>> {
        &mut self.textSearch
    }
    /// return number of dirs processed
    pub fn get_directories(&self) -> usize {
        self.num_directories
    }
    /// return number of files processed
    pub fn get_files(&self) -> usize {
        self.num_files
    }
    /// return patterns, e.g., file extensions to look for
    pub fn get_patterns(&self) -> &SearchPatterns {
        &self.patterns
    }

    /// add extention to search for - takes literal path
    pub fn add_pattern(&mut self, p: &Path) {
        self.patterns.push(p.to_path_buf());
    }
    /// reset to default state
    pub fn clear(&mut self) {
        self.patterns.clear();
        self.num_directories = 0;
        self.num_files = 0;
        self.textSearch = None;
        self.recurse = true;
    }

    fn visitDirectory(&mut self, directory: &Path) -> io::Result<()> {
        if let Some(textSearchObject) = &mut self.textSearch {
            textSearchObject.setSearchDirectory(&directory);
        }

        self.num_directories += 1;
        let mut sub_dirs = Vec::<PathBuf>::new();
        if directory.is_dir() {
            /* search local directory */
            for entry in fs::read_dir(directory)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    sub_dirs.push(path);  // save for processing after files
                } else {
                    self.num_files += 1;
                    if self.in_patterns(&entry) | self.patterns.is_empty() {

                        if let Some(textSearchObject) = &mut self.textSearch {
                            textSearchObject.searchFile(&Path::new(&entry.file_name()));
                        }
                    }
                }
            }
            /*-- recurse into subdirectories --*/
            for sub in sub_dirs {
                let mut pb = std::path::PathBuf::new();
                pb.push(sub);
                if self.recurse {
                    self.visitDirectory(&pb)?;
                }
            }
            return Ok(());  // normal return
        }
        Err(Error::new(ErrorKind::Other, "not a directory"))
    }

    pub fn in_patterns(&self, d: &DirEntry) -> bool {
        let p = d.path();
        let ext = p.extension();
        match ext {
            Some(extn) => self.patterns.contains(&PathBuf::from(extn)),
            // Some(extn) => self.pats.contains(&(extn.to_os_string())),
            None => false,
        }
    }
}


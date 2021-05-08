use colored::*;
use std::{usize, vec};

/// Workspace struct
pub struct Workspace {
    ///List of directories in the workspace
    dirs: Vec<String>,

    /// Name of the workspace
   pub name: String
}

impl Workspace {
    ///Adds a new directory to the workspace
    pub fn add_dir(&mut self, dir: String) {
        self.dirs.push(dir)
    }

    /// Checks if a directory exists in a workspace
    /// Returns position [`Option<usize>`] of the directory is exists
    /// Else return [`None`]
    fn check_dir_already_exists(&self, dir: &str) -> Option<usize> {
        self.dirs.iter().position(|x| x == dir)
    }

    pub fn remove_dir(&mut self, dir: &str) {
        let value = self.check_dir_already_exists(dir);
        
        match value {
            Some(index) => {
                self.dirs.remove(index);
                println!("Directory Removed from Workspace");
            }
            None => {
                println!("{}", "No such directory exists in workspace".to_string().bright_red());
            }
        }
    }

    pub fn new(name: String) -> Self {
        Workspace { dirs: vec![], name }
    }
}

#[cfg(test)]
mod tests {
    use crate::workspace::Workspace;
    #[test]
    fn add_directory() {
        let w = create_sample_workspace();

        assert_eq!(w.dirs.len(), 2);
    }

    fn create_sample_workspace() -> Workspace {
        let mut w = Workspace::new(String::from("Sample"));
        w.add_dir("Marcus".to_string());
        w.add_dir("Temple".to_string());

        w
    }

    #[test]
    fn remove_dir() {
        let mut w = create_sample_workspace();

        w.remove_dir("Temple");

        assert_eq!(w.dirs.len(), 1);
    }

    #[test]
    fn remove_non_existant_dir() {
        let mut w = create_sample_workspace();

        w.remove_dir("Workspace");

        assert_eq!(w.dirs.len(), 2);
    }
}

extern crate proton_cli;
extern crate tempdir;
extern crate git2;
extern crate rustc_serialize;

pub use std::{env, fs};
pub use std::fs::File;
pub use std::path::{Path, PathBuf};
pub use std::io::Read;

pub use git2::{Repository, Signature, Time};
pub use rustc_serialize::json;
pub use tempdir::TempDir;

pub use proton_cli::{Error, Project, initialize_project};

mod initialize_project {
    use super::*;

    #[test]
    fn works_with_an_empty_directory() {
        let (signature, root_dir) = setup();

        let root = root_dir.path();
        initialize_project(root, &signature).expect("Initialization failed");

        assert_initialized(root);
    }

    #[test]
    fn works_with_an_non_existent_directory() {
        let (signature, root_dir) = setup();

        let root = &(root_dir.path().join("nonexistent"));
        initialize_project(root, &signature).expect("Initialization failed");

        assert_initialized(root);
    }

    fn setup() -> (Signature<'static>, TempDir) {
        let signature = Signature::now("tester", "t@example.com").unwrap();
        let root_dir = TempDir::new("proton_cli_tests").unwrap();
        (signature, root_dir)
    }

    fn assert_initialized(root: &Path) {
        // Assert that protonfile exists
        let protonfile_path = root.join(Path::new("Protonfile.json"));
        assert!(protonfile_path.is_file(), "protonfile must exist");

        // Check that protonfile has right content
        assert_eq!(Project::empty(), File::open(&protonfile_path)
            .and_then(|mut protonfile| {
                let mut content = "".to_owned();
                protonfile.read_to_string(&mut content).map(|_| content)
            })
            .map_err(Error::Io)
            .and_then(|content| json::decode(&content).map_err(Error::JsonDecode))
            .expect("Loading protonfile into Project failed"));

        // Open the git repo and master branch
        let repo = Repository::open(root).unwrap();
        let commit = repo.refname_to_id("refs/heads/master")
            .and_then(|oid| repo.find_commit(oid))
            .expect("Finding master failed");
        let tree = commit.tree().expect("Opening master tree failed");

        // Assert master is correct
        assert!(0 == commit.parents().count(), "master must have 0 parents");
        assert!(tree.get_name("Protonfile.json").is_some(), "master must have protonfile");
    }
}

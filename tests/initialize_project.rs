extern crate proton_cli;
extern crate tempdir;
extern crate git2;
extern crate rustc_serialize;

mod common;

use std::fs::File;
use std::path::Path;

use git2::Repository;

use proton_cli::project_types::{Project, User, Permission, PermissionEnum};
use proton_cli::initialize_project;
use proton_cli::utils;

use common::rsa_keys::{self, TestKey};
use common::setup;


#[test]
fn works_with_an_empty_root() {
    let root_dir = setup::setup();
    let root = root_dir.path();
    try_initialize_project(&root);
}

#[test]
fn works_with_an_non_existent_root() {
    let root_dir = setup::setup();
    let root = &root_dir.path().join("nonexistent");
    try_initialize_project(&root);
}

fn try_initialize_project(root: &Path) {
    let admin_pub_key = rsa_keys::get_test_key(TestKey::AdminKeyPub);

    initialize_project(root, &admin_pub_key).expect("Initialization failed");

    assert_admin_created(root, &admin_pub_key);
    assert_initialized(root, &admin_pub_key);
}

#[test]
#[should_panic(expected = "Initialization failed")]
fn fails_with_a_non_empty_directory() {
    let root_dir = setup::setup();

    let root = root_dir.path();
    let admin_pub_key = rsa_keys::get_test_key(TestKey::AdminKeyPub);
    let _ = File::create(&root.join("unexpected")).expect("Making unexpected file failed");
    initialize_project(root, &admin_pub_key).expect("Initialization failed");
}

#[test]
#[should_panic(expect = "Initialization failed")]
fn fails_with_bad_key() {
    let root_dir = setup::setup();

    let root = root_dir.path();
    let admin_pub_key = rsa_keys::get_test_key(TestKey::BadPubKeyPub);
    initialize_project(root, &admin_pub_key).expect("Initialization failed");   
}

fn assert_admin_created<P: AsRef<Path>>(root: P, admin_pub_key: &str) {
    let project = utils::read_protonfile(Some(root.as_ref()))
        .expect("Loading project from file failed");
    let mut admin_user = User::new("admin".as_ref(), &admin_pub_key)
        .expect("Error creating admin user for comparison");
    let admin_permission = Permission::new(PermissionEnum::GrantPerm, None::<String>)
        .expect("Error creating default admin permission");
    admin_user.add_permission(admin_permission);
    assert_eq!(project.users.len(), 1);
    assert_eq!(project.users[0], admin_user);
}

fn assert_initialized(root: &Path, admin_pub_key: &str) {
    // Assert that protonfile exists
    let protonfile_path = root.join(Path::new("Protonfile.json"));
    assert!(protonfile_path.is_file(), "protonfile must exist");

    // Check that protonfile has right content
    assert_eq!(Project::empty(&admin_pub_key).expect("Creating empty project failed"), 
        utils::read_protonfile(Some(root)).expect("Reading protonfile failed"));

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

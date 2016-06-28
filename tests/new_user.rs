extern crate proton_cli;
extern crate tempdir;

mod common;

use common::rsa_keys::TestKey;
use common::setup;


/// Warning: This test changes env::current_directory
/// to better model new_user's expected use case.
/// Running tests with RUST_TEST_THREADS=1 runs tests
/// in serial, which avoids occasional false negatives
#[test]
fn works_with_new_and_existing_protonfile() {
    let root = setup::setup_init_cd();
    let admin_key_path = common::make_key_file(&root.path(), "admin.pem", TestKey::AdminKeyPem);

    setup::try_new_user(
        &admin_key_path.as_path(),
        root.path(),
        "Test User",
        "a.pub",
        TestKey::GoodKeyPub);
    setup::try_new_user(
        &admin_key_path.as_path(),
        root.path(),
        "Test User 2",
        "b.pub",
        TestKey::GoodKey2Pub);
}

#[test]
#[should_panic(expected = "not found")]
fn fails_with_a_nonexistent_protonfile() {
    // Don't initialize project (no protonfile created)
    let root_dir = setup::setup();
    let admin_key_path = common::make_key_file(&root_dir.path(), "admin.pem", TestKey::AdminKeyPem);

    setup::try_new_user(
        admin_key_path.as_path(),
        root_dir.path(),
        "Username",
        "a.pub",
        TestKey::GoodKeyPub);
}

#[test]
#[should_panic(expected = "Error adding user")]
fn fails_with_nonexistent_user_key_path() {
    let root = setup::setup_init_cd();

    let user_key_path = root.path().join("nonexistent");
    let admin_key_path = common::make_key_file(&root.path(), "admin.pem", TestKey::AdminKeyPem);

    match proton_cli::new_user(&admin_key_path.as_path(), &user_key_path.as_path(), "Username") {
        Ok(_) => (),
        Err(_) => panic!("Error adding user"),
    };
}

#[test]
#[should_panic(expected = "Public key is invalid")]
fn fails_with_non_pem_key() {
    let root = setup::setup_init_cd();
    let admin_key_path = common::make_key_file(&root.path(), "admin.pem", TestKey::AdminKeyPem);

    setup::try_new_user(
        admin_key_path.as_path(),
        root.path(),
        "Test User",
        "bad_pub_key.pub",
        TestKey::BadPubKeyPub);
}

/// Warning: This test changes env::current_directory
/// to better model new_user's expected use case.
/// Running tests with RUST_TEST_THREADS=1 runs tests
/// in serial, which avoids occasional false negatives
#[test]
#[should_panic(expected = "Duplicate user")]
fn fails_with_duplicate_user_key() {
    let root = setup::setup_init_cd();
    let admin_key_path = common::make_key_file(&root.path(), "admin.pem", TestKey::AdminKeyPem);

    setup::try_new_user(
        admin_key_path.as_path(),
        root.path(),
        "Test User 1",
        "a.pub",
        TestKey::GoodKeyPub);
    setup::try_new_user(
        admin_key_path.as_path(),
        root.path(),
        "Test User 2",
        "b.pub",
        TestKey::GoodKeyPub);
}

/// Warning: This test changes env::current_directory
/// to better model new_user's expected use case.
/// Running tests with RUST_TEST_THREADS=1 runs tests
/// in serial, which avoids occasional false negatives
#[test]
#[should_panic(expected = "Duplicate user")]
fn fails_with_duplicate_user_name() {
    let root = setup::setup_init_cd();
    let admin_key_path = common::make_key_file(&root.path(), "admin.pem", TestKey::AdminKeyPem);

    setup::try_new_user(
        admin_key_path.as_path(),
        root.path(),
        "Test User",
        "a.pub",
        TestKey::GoodKeyPub);
    setup::try_new_user(
        admin_key_path.as_path(),
        root.path(),
        "Test User",
        "b.pub",
        TestKey::GoodKey2Pub);
}


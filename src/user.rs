//! This module manages project users
use std::path::Path;

use dao::{ProtonDao};
use error::Error;
use project_types::User;
use utils;


/// Lookup and return a user from a public key
pub fn get_user<P: AsRef<Path>, PD: ProtonDao> (
    dao: &PD,
    public_key_path: P
) -> Result<User, Error> {
    
    let public_key_str = try!(utils::file_as_string(public_key_path.as_ref()));

    // Throw error if the given key is not a valid RSA public key
    if !utils::validate_rsa_pub_key(&public_key_str) {
        return Err(Error::InvalidPublicKey(public_key_str));
    }

    // Lookup uid
    let uid = try!(dao.get_user_id(&public_key_str));

    // Get user
    let user = try!(dao.get_user(uid));

    Ok(user)
}

pub fn new_user<PD: ProtonDao> (
    dao: &PD,
    name: &str
) -> Result<String, Error> {

    // Create keys
    let (user_pub_key, user_private_key) = try!(utils::create_pub_priv_keys());

    // Add user
    let _ = try!(dao.add_user(name, &user_private_key, &user_pub_key));

    // Return public key
    Ok(user_pub_key)
}

/// Removes a user
#[allow(unused_variables)]
pub fn remove_user<PD: ProtonDao> (
    dao: &PD,
    name: &str
) -> Result<(), Error> {

    Err(Error::TodoErr)
    // See if admin has permission to add user
    // Can't remove root
    // Remove user

}
/// Executable for proton_cli
extern crate rustc_serialize;
extern crate proton_cli;
extern crate docopt;

use std::env;
use std::path::Path;
use rustc_serialize::json;
use docopt::Docopt;

use proton_cli::error::Error;
use proton_cli::dao::{DaoPostgres, ProtonDao};
use proton_cli::project_types::{PermissionEnum, Project, Sequence, User};
use proton_cli::utils;


const USAGE: &'static str = "
Command-line interface for Proton

Usage:
  ./proton_cli delete-sequence <admin-key> <seqid>
  ./proton_cli get-layout-id <proj-name>
  ./proton_cli get-playlist-data <proj-name>
  ./proton_cli get-project <proj-name>
  ./proton_cli get-sequence <seqid>
  ./proton_cli get-user <public-key>
  ./proton_cli insert-sequence <admin-key> <proj-name> <seqid> [<index>]
  ./proton_cli list-permissions <uid>
  ./proton_cli new-layout <layout-file>
  ./proton_cli new-project <name> <layout-id>
  ./proton_cli new-section <admin-key> <t_start> <t_end> <seqid> <fixid>..
  ./proton_cli new-sequence <admin-key> <name> <music-file> <seq-duration> <layout-id>
  ./proton_cli new-user <admin-key> <name>
  ./proton_cli new-vixen-sequence <admin-key> <name> <music-file> <seq-duration> <frame-duration> <data-file> <layout-id>
  ./proton_cli patch-layout <admin-key> <layout-id> <patch-file>
  ./proton_cli remove-sequence <admin-key> <proj-name> <seqid>
  ./proton_cli remove-user <admin-key> <name>
  ./proton_cli set-permission <admin-key> (add | remove) <uid> Administrate
  ./proton_cli set-permission <admin-key> (add | remove) <uid> EditSequence <target-sequence>
  ./proton_cli set-permission <admin-key> (add | remove) <uid> EditSection <target-sequence> <target-section>
  ./proton_cli set-permission <admin-key> (add | remove) <name> EditSeqSec <target-section>
  ./proton_cli set-sequence-layout <admin-key> <seqid> <layout-id>
  ./proton_cli (-h | --help)

Options:
  -h --help     Show this screen
";

// Docopt arguments are mapped to this struct
#[derive(Debug, RustcDecodable)]
struct Args {
	arg_admin_key: Option<String>,
	arg_data_file: Option<String>,
	arg_fixid: Option<u32>,
	arg_frame_duration: Option<u32>,
	arg_index: Option<u32>,
	arg_layout_id: Option<u32>,
	arg_layout_file: Option<String>,
	arg_music_file: Option<String>,
	arg_name: Option<String>,
	arg_patch_file: Option<String>,
	arg_proj_name: Option<String>,
	arg_public_key: Option<String>,
	arg_root_public_key: Option<String>,
	arg_seqid: Option<u32>,
	arg_seq_duration: Option<u32>,
	arg_t_start: Option<u32>,
	arg_t_end: Option<u32>,
	arg_target_sequence: Option<u32>,
	arg_target_section: Option<u32>,
	arg_uid: Option<u32>,
}

// Generic return type of all functions that are called based on cli commands
enum ProtonReturn {
	LayoutId(u32),
	NoReturn,
	PlaylistData(String),
	Project(Project),
	PublicKey(String),
	Sequence(Sequence),
	SequenceId(u32),
	User(User),
}

// Entry point
fn main() {
	// Get command line arguments using Docopt. Provides guarantees about input types.
	let args: Args = Docopt::new(USAGE)
		.and_then(|d| d.decode())
		.unwrap_or_else(|e| e.exit());
	
	// Below unwrap()'s are safe within Docopt's usage rules

	// Create data access object for data retrieval from database, file, etc.
	// Use only Postgres for now
	let psql_dao = match DaoPostgres::new() {
		Ok(d) => d,
		Err(e) => panic!("Failed to retrieve DAO: {}", e)
	};

	// Every proton command is mapped to a specific function that should be run
	let command: fn(Args, DaoPostgres) -> Result<ProtonReturn, Error> = match env::args().nth(1).unwrap().as_ref() {
		"delete-sequence" => run_delete_sequence,
		"get-layout-id" => run_get_layout_id,
		"get-playlist-data" => run_get_playlist_data,
		"get-project" => run_get_project,
		"get-sequence" => run_get_sequence,
		"get-user" => run_get_user,
		"insert-sequence" => run_insert_sequence,
		"list-permissions" => run_list_permissions,
		"new-layout" => run_new_layout,
		"new-project" => run_new_project,
		"new-section" => run_new_section,
		"new-sequence" => run_new_sequence,
		"new-user" => run_new_user,
		"new-vixen-sequence" => run_new_vixen_sequence,
		"patch-layout" => run_patch_layout,
		"remove-sequence" => run_remove_sequence,
		"remove-user" => run_remove_user,
		"set-permission" => run_set_permission,
		"set-sequence-layout" => run_set_sequence_layout,
		_ => panic!("Invalid first argument"),
	};

	// Run the appropriate command and handle its return
	let result = command(args, psql_dao);
	match result {
		Ok(ret) => match ret {
			ProtonReturn::LayoutId(lid) => println!("Layout id: {}", lid),
			ProtonReturn::NoReturn => println!("Worked!"),
			ProtonReturn::PlaylistData(data) => println!("PLAYLIST_DATA:::{}", data),
			ProtonReturn::Project(project) => println!("Project: {:?}", project),
			ProtonReturn::PublicKey(s) => println!("PubKey: {}", s),
			ProtonReturn::Sequence(seq) => println!("Sequence: {:?}", seq),
			ProtonReturn::SequenceId(sid) => println!("Sequence id: {}", sid),
			ProtonReturn::User(user) => println!("User: {:?}", user)
		},
		Err(e) => {
			println!("Error: {:?}", e.to_string());
			std::process::exit(1);
		}
	};
}

fn run_delete_sequence<PD: ProtonDao>(args: Args, dao: PD) -> Result<ProtonReturn, Error> {
	let admin_key = args.arg_admin_key.unwrap();
	let admin_key_path = Path::new(&admin_key);
	let seqid = args.arg_seqid.unwrap();
	
	try!(proton_cli::delete_sequence(&dao, &admin_key_path, seqid));
	Ok(ProtonReturn::NoReturn)
}

/// get-layout-id <proj-name>
fn run_get_layout_id<PD: ProtonDao>(args: Args, dao: PD) -> Result<ProtonReturn, Error> {
	let proj_name = args.arg_proj_name.unwrap();
	let layout_id = try!(proton_cli::get_layout_id(&dao, &proj_name));
	Ok(ProtonReturn::LayoutId(layout_id))
}

/// get-playlist-data <proj-name>
fn run_get_playlist_data<PD: ProtonDao>(args: Args, dao: PD) -> Result<ProtonReturn, Error> {
	let proj_name = args.arg_proj_name.unwrap();
	let data = try!(proton_cli::get_playlist_data(&dao, &proj_name));
	Ok(ProtonReturn::PlaylistData(data))
}

/// get-project <proj-name>
fn run_get_project<PD: ProtonDao>(args: Args, dao: PD) -> Result<ProtonReturn, Error> {
	let proj_name = args.arg_proj_name.unwrap();
	let project = try!(proton_cli::get_project(&dao, &proj_name));
	Ok(ProtonReturn::Project(project))
}

/// get-sequence <seqid>
fn run_get_sequence<PD: ProtonDao>(args: Args, dao: PD) -> Result<ProtonReturn, Error> {
	let seqid = args.arg_seqid.unwrap();
	let sequence = try!(proton_cli::get_sequence(&dao, seqid));
	Ok(ProtonReturn::Sequence(sequence))
}

/// get-user <public-key>
fn run_get_user<PD: ProtonDao>(args: Args, dao: PD) -> Result<ProtonReturn, Error> {
	let public_key = args.arg_public_key.unwrap();
	let public_key_path = Path::new(&public_key);
	let user = try!(proton_cli::get_user(&dao, &public_key_path));
	Ok(ProtonReturn::User(user))
}

/// insert-sequence <admin-key> <proj-name> <seqid> [<index>]
fn run_insert_sequence<PD: ProtonDao>(args: Args, dao: PD) -> Result<ProtonReturn, Error> {
	let admin_key = args.arg_admin_key.unwrap();
	let admin_key_path = Path::new(&admin_key);
	let proj_name = args.arg_proj_name.unwrap();
	let seqid = args.arg_seqid.unwrap();
	let index = args.arg_index;

	let valid_permissions = vec![PermissionEnum::Administrate];
	let _ = try!(utils::check_valid_permission(
		&dao,
		admin_key_path,
		&valid_permissions));
	
	try!(proton_cli::insert_sequence(&dao, &proj_name, seqid, index));
	Ok(ProtonReturn::NoReturn)
}

/// list-permissions <uid>
fn run_list_permissions<PD: ProtonDao>(args: Args, dao: PD) -> Result<ProtonReturn, Error> {
	let uid = args.arg_uid.unwrap();
	let permissions = try!(proton_cli::get_permissions(&dao, uid));
	println!("{}", json::as_pretty_json(&permissions));
	Ok(ProtonReturn::NoReturn)
}

/// new-layout <layout-file>
fn run_new_layout<PD: ProtonDao>(args: Args, dao: PD) -> Result<ProtonReturn, Error> {
	let layout_file = args.arg_layout_file.unwrap();
	let layout_file_path = Path::new(&layout_file);
	let layout_id = try!(proton_cli::new_layout(&dao, &layout_file_path));
	Ok(ProtonReturn::LayoutId(layout_id))
}

/// new-project <name> <layout-id>
fn run_new_project<PD: ProtonDao>(args: Args, dao: PD) -> Result<ProtonReturn, Error> {
	let name = args.arg_name.unwrap();
	let layout_id = args.arg_layout_id.unwrap();
	let root_pub_key = try!(proton_cli::new_project(&dao, &name, layout_id));
	Ok(ProtonReturn::PublicKey(root_pub_key))
}

/// new-section <admin-key> <t_start> <t_end> <seqid> <fixid>..
#[allow(unused_variables)]
fn run_new_section<PD: ProtonDao>(args: Args, dao: PD) -> Result<ProtonReturn, Error> {
	Err(Error::TodoErr)
}

/// new-sequence <admin-key> <name> <music-file> <seq-duration> <layout-id>
fn run_new_sequence<PD: ProtonDao>(args: Args, dao: PD) -> Result<ProtonReturn, Error> {
	let admin_key = args.arg_admin_key.unwrap();
	let admin_key_path = Path::new(&admin_key);
	let name = args.arg_name.unwrap();
	let music_file = args.arg_music_file.unwrap();
	let music_file_path = Path::new(&music_file);
	let seq_duration = args.arg_seq_duration.unwrap();
	let layout_id = args.arg_layout_id;

	// Check that the admin has sufficient privileges
	let valid_permissions = vec![PermissionEnum::Administrate];
	let _ = try!(utils::check_valid_permission(
		&dao,
		admin_key_path,
		&valid_permissions));

	let seqid = try!(proton_cli::new_sequence(
		&dao,
		&name,
		&music_file_path,
		seq_duration,
		None::<u32>,
		layout_id));
	Ok(ProtonReturn::SequenceId(seqid))
}

/// new-user <admin-key> <name>
fn run_new_user<PD: ProtonDao>(args: Args, dao: PD) -> Result<ProtonReturn, Error> {
	let admin_key = args.arg_admin_key.unwrap();
	let admin_key_path = Path::new(&admin_key);
	let name = args.arg_name.unwrap();

	// See if admin has permission to add user
	let valid_permissions = vec![PermissionEnum::Administrate];
	let _ = try!(utils::check_valid_permission(
		&dao,
		admin_key_path,
		&valid_permissions));

	let public_key = try!(proton_cli::new_user(&dao, &name));
	Ok(ProtonReturn::PublicKey(public_key))
}

/// new-vixen-sequence <admin-key> <name> <music-file> <seq-duration> <frame-duration> <data-file> <layout-id>
fn run_new_vixen_sequence<PD: ProtonDao>(args: Args, dao: PD) -> Result<ProtonReturn, Error> {
	let admin_key = args.arg_admin_key.unwrap();
	let admin_key_path = Path::new(&admin_key);
	let name = args.arg_name.unwrap();
	let music_file = args.arg_music_file.unwrap();
	let music_file_path = Path::new(&music_file);
	let seq_duration = args.arg_seq_duration.unwrap();
	let frame_duration = args.arg_frame_duration.unwrap();
	let data_file = args.arg_data_file.unwrap();
	let data_file_path = Path::new(&data_file);
	let layout_id = match args.arg_layout_id {
		Some(lid) => lid,
		None => {
			let default_layout = try!(dao.get_default_layout());
			default_layout.layout_id
		},
	};

	// Check that the admin has sufficient privileges
	let valid_permissions = vec![PermissionEnum::Administrate];
	let _ = try!(utils::check_valid_permission(
		&dao,
		admin_key_path,
		&valid_permissions));

	let seqid = try!(proton_cli::new_vixen_sequence(
		&dao,
		&name,
		&music_file_path,
		seq_duration,
		frame_duration,
		&data_file_path,
		layout_id));
	Ok(ProtonReturn::SequenceId(seqid))
}

/// patch-layout <admin-key> <layout-id> <patch-file>
fn run_patch_layout<PD: ProtonDao>(args: Args, dao: PD) -> Result<ProtonReturn, Error> {
	let admin_key = args.arg_admin_key.unwrap();
	let admin_key_path = Path::new(&admin_key);
	let layout_id = args.arg_layout_id.unwrap();
	let patch_file = args.arg_patch_file.unwrap();
	let patch_file_path = Path::new(&patch_file);

	// Check that the admin has sufficient privileges
	let valid_permissions = vec![PermissionEnum::Administrate];
	let _ = try!(utils::check_valid_permission(
		&dao,
		admin_key_path,
		&valid_permissions));

	try!(proton_cli::patch_layout(
		&dao,
		layout_id,
		&patch_file_path));
	
	Ok(ProtonReturn::NoReturn)
}

/// remove-sequence <admin-key> <proj-name> <seqid>
fn run_remove_sequence<PD: ProtonDao>(args: Args, dao: PD) -> Result<ProtonReturn, Error> {
	let admin_key = args.arg_admin_key.unwrap();
	let admin_key_path = Path::new(&admin_key);
	let proj_name = args.arg_proj_name.unwrap();
	let seqid = args.arg_seqid.unwrap();

	// Check that the admin has sufficient privileges
	let valid_permissions = vec![PermissionEnum::Administrate, PermissionEnum::EditSequence(seqid)];
	let _ = try!(utils::check_valid_permission(
		&dao,
		admin_key_path,
		&valid_permissions));

	try!(proton_cli::remove_sequence(&dao, &proj_name, seqid));
	Ok(ProtonReturn::NoReturn)
}

/// remove-user <admin-key> <uid>
fn run_remove_user<PD: ProtonDao>(args: Args, dao: PD) -> Result<ProtonReturn, Error> {
	let name = args.arg_name.unwrap();
	try!(proton_cli::remove_user(&dao, &name));
	Ok(ProtonReturn::NoReturn)
}

/// set-permission <admin-key> (add | remove) <uid> Administrate
/// set-permission <admin-key> (add | remove) <uid> EditSequence <target-sequence>
/// set-permission <admin-key> (add | remove) <uid> EditSection <target-sequence> <target-section>
/// set-permission <admin-key> (add | remove) <name> EditSeqSec <target-section>
fn run_set_permission<PD: ProtonDao>(args: Args, dao: PD) -> Result<ProtonReturn, Error> {
	let admin_key = args.arg_admin_key.unwrap();
	let admin_key_path = Path::new(&admin_key);
	let added = env::args().nth(3).unwrap() == "add";
	let uid = args.arg_uid.unwrap();
	let permission_name = env::args().nth(5).unwrap();
	let target_sequence = args.arg_target_sequence;
	let target_section = args.arg_target_section;

	try!(proton_cli::set_permission(
		&dao,
		&admin_key_path,
		added,
		uid,
		&permission_name,
		target_sequence,
		target_section));
	Ok(ProtonReturn::NoReturn)
}

/// set-sequence-layout <admin-key> <seqid> <layout-id>
fn run_set_sequence_layout<PD: ProtonDao>(args: Args, dao: PD) -> Result<ProtonReturn, Error> {
	let admin_key = args.arg_admin_key.unwrap();
	let admin_key_path = Path::new(&admin_key);
	let seqid = args.arg_seqid.unwrap();
	let layout_id = args.arg_layout_id.unwrap();

	// Check that the admin has sufficient privileges
	let valid_permissions = vec![PermissionEnum::Administrate];
	let _ = try!(utils::check_valid_permission(
		&dao,
		admin_key_path,
		&valid_permissions));

	try!(proton_cli::set_sequence_layout(
		&dao,
		layout_id,
		seqid));
	Ok(ProtonReturn::NoReturn)
}

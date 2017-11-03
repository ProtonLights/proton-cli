use postgres::{Connection, TlsMode};

use dao::ConnectionConfig;
use error::Error;


pub struct DaoPostgres {
    pub conn: Connection
}

pub type ChannelDaoPostgres = DaoPostgres;
pub type DataDaoPostgres = DaoPostgres;
pub type FixtureDaoPostgres = DaoPostgres;
pub type LayoutDaoPostgres = DaoPostgres;
pub type PermissionDaoPostgres = DaoPostgres;
pub type ProjectDaoPostgres = DaoPostgres;
pub type SectionDaoPostgres = DaoPostgres;
pub type SequenceDaoPostgres = DaoPostgres;
pub type UserDaoPostgres = DaoPostgres;


impl DaoPostgres {
    pub fn new() -> Result<DaoPostgres, Error> {
        let conn = try!(get_connection());
        Ok(DaoPostgres {
            conn: conn
        })
    }
}

/// Gets a new connection to the postgresql database
fn get_connection() -> Result<Connection, Error> {
    // Load connection info from file
    // NEVER commit this file; it contains the password
    let connection_config = try!(ConnectionConfig::load());
    let password = connection_config.password;
    let host = connection_config.host;

    // Build connection string
    let connection_string = format!("postgresql://proton:{}@{}/proton_cli", password, host);

    // Connect
    Connection::connect(connection_string, TlsMode::None)
        .map_err(Error::PostgresConnection)
}


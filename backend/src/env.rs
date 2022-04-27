use std::env::{set_var, var_os};
use std::ffi::OsString;
use std::net::SocketAddr;

/*
TODO @ jacob:
- Migrate this implementation to use `impl Env` and add methods, one for each variant, to access the associated environment variable using (var_os)
- Find a way to associate an error enum with the Env variant
*/

const EG_CONN_STR: &str = "";

#[derive(strum::Display, Debug)]
#[allow(dead_code)]
// Map environment variables to Rust enums
pub enum Env {
    #[strum(serialize = "RUST_LOG")]
    RustLog,
    #[strum(serialize = "APP_HOST_NAME")]
    AppHost,
    #[strum(serialize = "APP_PORT")]
    AppPort,
    #[strum(serialize = "DATABASE_URL")]
    DbURL,
    #[strum(serialize = "DB_HOST_NAME")]
    DbAddr,
    #[strum(serialize = "DB_PORT_HOST")]
    DbPort,
    #[strum(serialize = "POSTGRES_DB")]
    DbName,
    #[strum(serialize = "POSTGRES_USER")]
    DbUser,
    #[strum(serialize = "POSTGRES_PASSWORD")]
    DbPassword,
}

#[derive(Debug)]
pub enum EnvError {
    Missing(String),
    Invalid(String),
}
impl std::fmt::Display for EnvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Missing(s) => write!(f, "Env Variable Misisng: {}", s),
            Self::Invalid(s) => write!(f, "Env Variable Invalid: {}", s),
        }
    }
}
impl std::error::Error for EnvError {}

// Ignore the return value, but pass along the error String
pub fn require_env_vars() -> Result<(), anyhow::Error> {
    get_rust_log()?;
    if get_db_url().is_ok() {
        return Ok(());
    }
    get_app_host().map(|_| ())?;
    get_app_port().map(|_| ())?;
    get_db_addr().map(|_| ())?;
    get_db_port().map(|_| ())?;
    get_db_name().map(|_| ())?;
    get_db_user().map(|_| ())?;
    get_db_password().map(|_| ())?;
    Ok(()) // All required environment variables are present
}

pub fn get_rust_log() -> Result<(), anyhow::Error> {
    if var_os(Env::RustLog.to_string()).is_none() {
        set_var(Env::RustLog.to_string(), "debug")
    }
    Ok(())
}

pub fn get_app_url() -> Result<SocketAddr, anyhow::Error> {
    let host = get_app_host()?.into_string().map_err(|_| {
        EnvError::Invalid(format!("Failed to convert {} into string", Env::AppHost))
    })?;

    let port = get_app_port()?.into_string().map_err(|_| {
        EnvError::Invalid(format!("Failed to convert {} into string", Env::AppPort))
    })?;

    let addr = format!("{}:{}", host, port);

    addr.parse::<SocketAddr>().map_err(|err| {
        EnvError::Invalid(format!(
            "Failed to parse host:port ({:?}:{:?}) into a SockerAddr. Err = {err}",
            host, port
        ))
        .into()
    })
}

pub fn get_app_host() -> Result<OsString, anyhow::Error> {
    var_os(Env::AppHost.to_string()).ok_or_else(|| {
        EnvError::Missing(format!(
            "Server cannot start without {} set to a valid IP or alias (e.g. localhost or 0.0.0.0)",
            Env::AppHost
        ))
        .into()
    })
}

pub fn get_app_port() -> Result<OsString, anyhow::Error> {
    var_os(Env::AppPort.to_string()).ok_or_else(|| {
        EnvError::Missing(format!(
            "Backend server cannot start without {} set to an open port",
            Env::AppPort,
        ))
        .into()
    })
}

pub fn get_db_url() -> Result<String, anyhow::Error> {
    if let Some(db_url) = var_os(Env::DbURL.to_string()) {
        let db_url = db_url
            .into_string()
            .map_err(|_| EnvError::Invalid(format!("{} invalid utf8", Env::DbPort)))?;
        return Ok(db_url);
    }
    let addr = get_db_addr()?
        .into_string()
        .map_err(|_| EnvError::Invalid(format!("{} is invalid utf8", Env::DbAddr)))?;

    let port = get_db_port()?
        .into_string()
        .map_err(|_| EnvError::Invalid(format!("{} invalid utf8", Env::DbPort)))?;

    let name = get_db_name()?
        .into_string()
        .map_err(|_| EnvError::Invalid(format!("{} invalid utf8", Env::DbName)))?;

    let user = get_db_user()?
        .into_string()
        .map_err(|_| EnvError::Invalid(format!("{} invalid utf8", Env::DbUser)))?;

    let password = get_db_password()?
        .into_string()
        .map_err(|_| EnvError::Invalid(format!("{} invalid utf8", Env::DbPassword)))?;

    let url = format!("postgres://{user}:{password}@{addr}:{port}/{name}");
    Ok(url)
}

pub fn get_db_addr() -> Result<OsString, anyhow::Error> {
    var_os(Env::DbAddr.to_string()).ok_or_else(|| {
        EnvError::Missing(format!(
            "Database host (env var {}) is required to connect to the database e.g. {}",
            Env::DbAddr,
            EG_CONN_STR
        ))
        .into()
    })
}
pub fn get_db_port() -> Result<OsString, anyhow::Error> {
    var_os(Env::DbPort.to_string()).ok_or_else(|| {
        EnvError::Missing(format!(
            "Database post (env var {}) is required to connect to the database e.g {}",
            Env::DbPort,
            EG_CONN_STR
        ))
        .into()
    })
}
pub fn get_db_name() -> Result<OsString, anyhow::Error> {
    var_os(Env::DbName.to_string()).ok_or_else(|| {
        EnvError::Missing(format!(
            "Database name (env var {}) is required to connect to the database {}",
            Env::DbName,
            EG_CONN_STR
        ))
        .into()
    })
}
pub fn get_db_user() -> Result<OsString, anyhow::Error> {
    var_os(Env::DbUser.to_string()).ok_or_else(|| {
        EnvError::Missing(format!(
            "Database user (env var {}) is required to connect to the database {}",
            Env::DbUser,
            EG_CONN_STR
        ))
        .into()
    })
}
pub fn get_db_password() -> Result<OsString, anyhow::Error> {
    var_os(Env::DbPassword.to_string()).ok_or_else(|| {
        EnvError::Missing(format!(
            "Database password (env var {}) is required to connect to the database {}",
            Env::DbPassword,
            EG_CONN_STR
        ))
        .into()
    })
}

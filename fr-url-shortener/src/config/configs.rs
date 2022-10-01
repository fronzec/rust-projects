use std::env;
use std::io::{Error, ErrorKind};

pub fn log_envs() -> Result<(), Error> {
    let user = env::var("APP_DB_USER");
    let password = env::var("APP_DB_PASSWORD");
    let db = env::var("APP_DB_NAME");

    if user.is_err() || password.is_err() || db.is_err() {
        println!("---->(config) one or more required envvar(s) not found, exiting");
        return Err(Error::new(ErrorKind::Other, "some config not found!!!"));
    }

    if user.is_ok() {
        println!("---->(config) user <{}>", user.unwrap());
    }
    if password.is_ok() {
        println!("---->(config) password <***>")
    }
    if db.is_ok() {
        println!("---->(config) db <{}>", db.unwrap())
    }
    Ok(())
}
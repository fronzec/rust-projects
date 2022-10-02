use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind};

use java_properties::{PropertiesError, PropertiesIter};
use java_properties::PropertiesWriter;
use java_properties::read;
use java_properties::write;

pub fn load_configs() -> Result<(), Error> {
    let user = env::var("APP_DB_USER");
    let password = env::var("APP_DB_PASSWORD");
    let db = env::var("APP_DB_NAME");
    const DEFAULT_CONFIG_FILE: &str = "./config/latest/configurations.properties";

    if user.is_err() || password.is_err() || db.is_err() {
        println!("---->(config) one or more required envvar(s) not found, exiting");
        return Err(Error::new(ErrorKind::Other, "some config not found!!!"));
    }

    if user.is_ok() {
        println!("---->(config) user ok <{}>", user.unwrap());
    }
    if password.is_ok() {
        println!("---->(config) password ok <***>")
    }
    if db.is_ok() {
        println!("---->(config) db ok <{}>", db.unwrap())
    }
    let prop_file_env = env::var("CONFIG_FILE_PATH");
    let mut config_file = DEFAULT_CONFIG_FILE.to_string();
    if prop_file_env.is_ok() {
        println!("--->(config) loading properties file from custom route");
        config_file = prop_file_env.unwrap();
    }
    let result = load_props_from_file(&config_file);
    match result {
        Ok(configs) => {
            for (key, value) in &configs {
                println!("{}:{}", key, value);
            }
        }
        Err(error) => {
            println!("failed to load configs from file <{}>, detail= {}", config_file, error.to_string());
            return Err(Error::new(ErrorKind::Other, "configuration file cannot be read"));
        }
    }

    Ok(())
}

/// Load configs from the specified file.
fn load_props_from_file(file: &String) -> Result<HashMap<String, String>, PropertiesError> {
    let mut f = File::open(file)?;
    let mut configs = HashMap::new();
    PropertiesIter::new(BufReader::new(f)).read_into(|k, v| {
        configs.insert(k, v);
    })?;
    return Ok(configs);
}
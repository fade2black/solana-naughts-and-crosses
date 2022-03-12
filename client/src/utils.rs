use crate::{Error, Result};
//use borsh::BorshSerialize;
use solana_sdk::signer::keypair::{read_keypair_file, Keypair};
use yaml_rust::YamlLoader;

fn get_config() -> Result<yaml_rust::Yaml> {
    let path = match home::home_dir() {
        Some(mut path) => {
            path.push(".config/solana/cli/config.yml");
            path
        }
        None => {
            return Err(Error::ConfigReadError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "failed to locate homedir and thus can not locate solana config",
            )));
        }
    };
    let config = std::fs::read_to_string(path).map_err(|e| Error::ConfigReadError(e))?;
    let mut config = YamlLoader::load_from_str(&config)?;
    match config.len() {
        1 => Ok(config.remove(0)),
        l => Err(Error::InvalidConfig(format!(
            "expected one yaml document got ({})",
            l
        ))),
    }
}

pub fn get_rpc_url() -> Result<String> {
    let config = get_config()?;
    match config["json_rpc_url"].as_str() {
        Some(s) => Ok(s.to_string()),
        None => {
            return Err(Error::InvalidConfig(
                "missing `json_rpc_url` field".to_string(),
            ))
        }
    }
}

/// Gets the local solana wallet that has been configured
/// on the machine.
pub fn get_user_keypair(path_key: &str) -> Result<Keypair> {
    let config = get_config()?;

    let path = match config[path_key].as_str() {
        Some(s) => s,
        None => {
            return Err(Error::InvalidConfig(format!(
                "missing `{}` field",
                path_key
            )))
        }
    };

    read_keypair_file(path).map_err(|e| {
        Error::InvalidConfig(format!("failed to read keypair file ({}): ({})", path, e))
    })
}

pub fn get_program_keypair(keypair_path: &str) -> Result<Keypair> {
    let program_keypair = read_keypair_file(keypair_path).map_err(|e| {
        Error::InvalidConfig(format!(
            "failed to read program keypair file ({}): ({})",
            keypair_path, e
        ))
    })?;

    Ok(program_keypair)
}
